use crate::{error::Result, parse_text, BootconfigParser, Rule};
use pest::Parser;
use snafu::{ensure_whatever, OptionExt, ResultExt};
use std::collections::BTreeSet;

/// Merge single-line bootconfig drop-ins before generating binary bootconfig.
///
/// `kernel.module_blacklist` contributions are combined into one quoted,
/// comma-separated value, with module names sorted and deduplicated.
/// Linux also accepts `module-blacklist`: its parameter matcher treats hyphens
/// and underscores alike (Linux v6.12, kernel/params.c, parameq).
/// Merge both spellings so neither can override the other's contributions,
/// and always emit the documented `kernel.module_blacklist` spelling. Other
/// assignments retain their order and Linux's duplicate-assignment checks.
/// Module names must be nonempty ASCII letters, digits or underscores; empty
/// contributions cannot erase another package's blacklist.
/// This must run before the SDK bootconfig tool, not while predicting a
/// command line from an already generated binary.
pub fn merge_dropins(text: &str) -> Result<String> {
    let mut output = String::new();
    let mut modules = BTreeSet::new();
    let mut blacklist_position = None;
    for line in text.lines() {
        let blacklist = line.split_once('=').is_some_and(|(key, _)| {
            matches!(
                key.trim(),
                "kernel.module_blacklist" | "kernel.module-blacklist"
            )
        });
        if !blacklist {
            output.push_str(line);
            output.push('\n');
            continue;
        }
        let config = parse_text(line)?;
        let parsed = BootconfigParser::parse(Rule::config, line)
            .whatever_context("invalid module blacklist drop-in")?
            .next()
            .expect("grammar requires config");
        let mut statements = parsed.into_inner().filter(|p| p.as_rule() != Rule::EOI);
        ensure_whatever!(
            statements.next().is_some_and(|p| p.as_rule() == Rule::pair)
                && statements.next().is_none(),
            "module blacklist drop-in must contain only its assignment"
        );
        blacklist_position.get_or_insert(output.len());
        for parameter in config.kernel {
            let value = parameter
                .value
                .whatever_context("module blacklist requires a value")?;
            for module in value.split(',') {
                let module = module.trim();
                ensure_whatever!(
                    !module.is_empty()
                        && module
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'_'),
                    "invalid module blacklist name '{module}'"
                );
                modules.insert(module.to_owned());
            }
        }
    }
    if let Some(position) = blacklist_position {
        let value = modules.into_iter().collect::<Vec<_>>().join(",");
        // Quoting is essential: an unquoted bootconfig array would emit a
        // separate command-line assignment for each module.
        output.insert_str(
            position,
            &format!("kernel.module_blacklist = \"{value}\"\n"),
        );
    }
    // Do not silently accept conflicting non-list settings or invalid syntax.
    let config = parse_text(&output)?;
    let blacklists = config
        .kernel
        .iter()
        .filter(|p| matches!(p.name.as_str(), "module_blacklist" | "module-blacklist"))
        .collect::<Vec<_>>();
    ensure_whatever!(
        blacklists.is_empty()
            || (blacklist_position.is_some()
                && blacklists.len() == 1
                && blacklists[0].value.is_some()),
        "module blacklist must be a single-line assignment"
    );
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format_params;

    #[test]
    fn combines_package_blacklists_into_one_kernel_argument() {
        let input = concat!(
            "kernel.module_blacklist = i8042\n",
            "kernel.fips = 1\n",
            "kernel.module_blacklist = binfmt_misc\n",
            "kernel.module_blacklist = \"i8042,binfmt_misc\"\n",
            "kernel.sysctl.kernel.kexec_load_disabled = 1\n",
            "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
            "init.systemd.unit = fipscheck.target\n",
        );
        let merged = merge_dropins(input).unwrap();
        let config = parse_text(&merged).unwrap();
        assert_eq!(
            config.uki_cmdline("root=/dev/dm-0", "").unwrap(),
            concat!(
                "module_blacklist=binfmt_misc,i8042 fips=1 ",
                "sysctl.kernel.kexec_load_disabled=1 ",
                "systemd.mask=proc-sys-fs-binfmt_misc.mount ",
                "root=/dev/dm-0 -- systemd.unit=fipscheck.target"
            )
        );
        assert_eq!(merge_dropins(&merged).unwrap(), merged);
    }

    #[test]
    fn merges_kernel_parameter_alias_without_losing_contributions() {
        let merged = merge_dropins(concat!(
            "kernel.module-blacklist = i8042\n",
            "kernel.module_blacklist = binfmt_misc\n",
        ))
        .unwrap();
        assert_eq!(merged, "kernel.module_blacklist = \"binfmt_misc,i8042\"\n");
        assert_eq!(
            format_params(&parse_text(&merged).unwrap().kernel).unwrap(),
            "module_blacklist=binfmt_misc,i8042 "
        );
    }

    #[test]
    fn preserves_other_arrays_and_namespaces() {
        let input = concat!(
            "kernel.console = tty0, ttyS0\n",
            "kernel.module_blacklist = i8042, binfmt_misc, i8042\n",
            "init.module_blacklist = userspace\n",
        );
        let config = parse_text(&merge_dropins(input).unwrap()).unwrap();
        assert_eq!(
            format_params(&config.kernel).unwrap(),
            "console=tty0 console=ttyS0 module_blacklist=binfmt_misc,i8042 "
        );
        assert_eq!(
            format_params(&config.init).unwrap(),
            "module_blacklist=userspace "
        );
    }

    #[test]
    fn rejects_empty_or_malformed_modules_and_conflicting_settings() {
        for input in [
            "kernel.module_blacklist =",
            "kernel.module_blacklist = \"i8042,,binfmt_misc\"",
            "kernel.module_blacklist = \"i8042 binfmt_misc\"",
            "kernel.module_blacklist = ../binfmt_misc",
            "kernel.module_blacklist = i8042; init.extra = value",
            "kernel.module_blacklist = i8042; other.extra = value",
            "kernel.module_blacklist += binfmt_misc",
            "kernel.module_blacklist",
            "kernel { module_blacklist = i8042 }",
            "kernel.fips = 1\nkernel.fips = 0",
        ] {
            assert!(merge_dropins(input).is_err(), "{input}");
        }
    }
}
