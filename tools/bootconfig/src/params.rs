//! The bootconfig parameters a built image carries, how they render onto a
//! kernel command line, and how the drop-ins packages contribute are merged
//! into them.

use crate::error::{self, Result};
use crate::parsers::bootconfig::parse_assignments;
use snafu::ensure;
use std::collections::{BTreeMap, BTreeSet};

/// Keys whose drop-in contributions combine into one comma-separated value.
const LIST_KEYS: &[&str] = &["kernel.initcall_blacklist", "kernel.module_blacklist"];

/// One boot parameter. Parsing rejects a key without a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) value: String,
}

/// Parsed bootconfig parameters split by prefix.
#[derive(Debug, Default)]
pub(crate) struct BootconfigParams {
    /// kernel.* parameters (prefix stripped).
    pub(crate) kernel: Vec<Parameter>,
    /// init.* parameters (prefix stripped).
    pub(crate) init: Vec<Parameter>,
}

impl BootconfigParams {
    /// Compose the command line from these parameters and the base arguments
    /// of the image, as Linux composes `/proc/cmdline`.
    pub(crate) fn cmdline(&self, kernel: &str, init: &str) -> Result<String> {
        let kernel = format!("{}{kernel}", format_params(&self.kernel)?);
        let init = format!("{}{init}", format_params(&self.init)?);
        Ok([kernel.trim(), "--", init.trim()]
            .into_iter()
            .filter(|arguments| !arguments.is_empty())
            .collect::<Vec<_>>()
            .join(" "))
    }
}

/// Format bootconfig parameters as kernel command line fragment.
///
/// Uses xbc_snprint_cmdline() rules: `key=value ` with trailing space,
/// values with whitespace are quoted.
pub(crate) fn format_params(params: &[Parameter]) -> Result<String> {
    params
        .iter()
        .map(|Parameter { name, value }| {
            ensure!(!value.contains('"'), error::UnsupportedQuoteSnafu { name });
            Ok(if value.contains([' ', '\t', '\r', '\n']) {
                format!("{name}=\"{value}\" ")
            } else {
                format!("{name}={value} ")
            })
        })
        .collect()
}

/// Merge the drop-in contributions to each list key into one sorted, deduplicated,
/// comma-separated value, keeping every other assignment as written.
pub(crate) fn merge_parameters(text: &str) -> Result<String> {
    let mut lists: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut merged = Vec::new();
    for assignment in parse_assignments(text)? {
        if !LIST_KEYS.contains(&assignment.name.as_str()) {
            let written = assignment.written();
            merged.push((assignment.name, written));
            continue;
        }
        ensure!(
            assignment.values().all(|value| !value.contains('"')),
            error::UnsupportedQuoteSnafu {
                name: &assignment.name
            }
        );
        // A comma-separated value is a list whether it was written as an
        // array or as one quoted value, so contributions combine either way.
        let values: Vec<String> = assignment
            .values()
            .flat_map(|value| value.split(','))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect();
        lists.entry(assignment.name).or_default().extend(values);
    }

    // The merged value is quoted because an unquoted comma-separated value is a
    // bootconfig array, which the kernel expands back into one command-line
    // assignment per value.
    merged.extend(
        lists
            .into_iter()
            .map(|(name, values)| (name, format!("\"{}\"", Vec::from_iter(values).join(",")))),
    );
    // A stable sort keeps a key assigned twice in order, for `bootconfig -a` to reject.
    merged.sort_by(|(a, _), (b, _)| a.cmp(b));
    Ok(merged
        .into_iter()
        .map(|(name, value)| format!("{name} = {value}\n"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::bootconfig::parse_text;
    use crate::Error;
    use test_case::test_case;

    fn bootconfig(text: &str) -> BootconfigParams {
        parse_text(text).unwrap()
    }

    #[test]
    fn kernel_and_init_namespaces_stay_on_their_side_of_the_separator() {
        let params = bootconfig("kernel { console=tty0,ttyS0 }\ninit.systemd.unit=rescue.target");
        assert_eq!(
            params.cmdline("root=/dev/dm-0", "quiet").unwrap(),
            "console=tty0 console=ttyS0 root=/dev/dm-0 -- systemd.unit=rescue.target quiet"
        );
    }

    #[test]
    fn test_merge_combines_contributions_to_a_key() {
        let input = concat!(
            "kernel.module_blacklist = i8042\n",
            "kernel.fips = 1\n",
            "kernel.console = tty0, ttyS0\n",
            "kernel.module_blacklist = binfmt_misc\n",
            "kernel.module_blacklist = \"i8042,binfmt_misc\"\n",
            "kernel.sysctl.kernel.kexec_load_disabled = 1\n",
            "init.systemd.unit = fipscheck.target\n",
        );
        let merged = merge_parameters(input).unwrap();
        assert_eq!(
            merged,
            concat!(
                "init.systemd.unit = fipscheck.target\n",
                "kernel.console = tty0, ttyS0\n",
                "kernel.fips = 1\n",
                "kernel.module_blacklist = \"binfmt_misc,i8042\"\n",
                "kernel.sysctl.kernel.kexec_load_disabled = 1\n",
            )
        );
        assert_eq!(
            parse_text(&merged)
                .unwrap()
                .cmdline("root=/dev/dm-0", "")
                .unwrap(),
            concat!(
                "console=tty0 console=ttyS0 ",
                "fips=1 ",
                "module_blacklist=binfmt_misc,i8042 ",
                "sysctl.kernel.kexec_load_disabled=1 ",
                "root=/dev/dm-0 -- systemd.unit=fipscheck.target",
            )
        );
        // Merging is idempotent, so an image can be rebuilt from its input.
        assert_eq!(merge_parameters(&merged).unwrap(), merged);
    }

    #[test_case(
        "kernel.module-blacklist = i8042\nkernel.module_blacklist = binfmt_misc\n",
        "kernel.module-blacklist = i8042\nkernel.module_blacklist = \"binfmt_misc\"\n"
        ; "the_two_spellings_of_a_key_stay_apart"
    )]
    #[test_case(
        "kernel.console = tty0, ttyS0\n",
        "kernel.console = tty0, ttyS0\n"
        ; "an_array_stays_an_array"
    )]
    #[test_case(
        "kernel.earlycon = \"uart8250,io,0x3f8,115200\"\n",
        "kernel.earlycon = \"uart8250,io,0x3f8,115200\"\n"
        ; "a_quoted_comma_stays_in_its_value"
    )]
    #[test_case(
        "init.x = 'a b', \"c\", d\n",
        "init.x = 'a b', \"c\", d\n"
        ; "each_element_keeps_its_quotes"
    )]
    #[test_case(
        "kernel.SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25\n",
        "kernel.SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST = 25\n"
        ; "an_unspaced_assignment"
    )]
    #[test_case(
        "kernel.fips = 1 # FIPS\n",
        "kernel.fips = 1\n"
        ; "a_trailing_comment_is_dropped"
    )]
    #[test_case(
        "kernel.x = \x0ca, b\x0b\n",
        "kernel.x = a, b\n"
        ; "the_edges_the_kernel_strips_are_dropped"
    )]
    #[test_case(
        "kernel.initcall_blacklist = vmd_drv_init, megasas_init, mpt3sas_init, pqi_init\n",
        "kernel.initcall_blacklist = \"megasas_init,mpt3sas_init,pqi_init,vmd_drv_init\"\n"
        ; "an_initcall_blacklist_array_becomes_one_value"
    )]
    #[test_case(
        "init.module_blacklist = a, b\n",
        "init.module_blacklist = a, b\n"
        ; "a_list_key_outside_the_kernel_namespace_is_kept"
    )]
    #[test_case(
        "kernel.fips = 1\nkernel.fips = 0\n",
        "kernel.fips = 1\nkernel.fips = 0\n"
        ; "a_key_assigned_twice_is_left_for_bootconfig_to_reject"
    )]
    #[test_case(
        "kernel { module_blacklist = i8042; fips = 1 }\nkernel.module_blacklist = binfmt_misc\n",
        "kernel.fips = 1\nkernel.module_blacklist = \"binfmt_misc,i8042\"\n"
        ; "a_block_contributes_like_a_dotted_key"
    )]
    #[test_case(
        "kernel.module_blacklist = i8042; init.extra = value\n",
        "init.extra = value\nkernel.module_blacklist = \"i8042\"\n"
        ; "a_shared_line_keeps_both_assignments"
    )]
    #[test_case(
        "# comment\nkernel {}\ninit {}\n",
        ""
        ; "a_namespace_without_values_contributes_nothing"
    )]
    #[test_case("", "" ; "nothing_to_merge")]
    fn test_merge(input: &str, expected: &str) {
        let merged = merge_parameters(input).unwrap();
        assert_eq!(merged, expected);
        assert_eq!(merge_parameters(&merged).unwrap(), merged);
    }

    #[test_case(
        "kernel.module_blacklist = ../binfmt_misc",
        "kernel.module_blacklist = \"../binfmt_misc\"\n"
        ; "a_module_the_kernel_will_not_find"
    )]
    #[test_case(
        "kernel.module_blacklist = \"i8042 binfmt_misc\"",
        "kernel.module_blacklist = \"i8042 binfmt_misc\"\n"
        ; "a_value_holding_a_space"
    )]
    fn test_merge_leaves_values_for_the_kernel_to_judge(input: &str, expected: &str) {
        let merged = merge_parameters(input).unwrap();
        assert_eq!(merged, expected);
        assert_eq!(merge_parameters(&merged).unwrap(), merged);
    }

    #[test]
    fn test_merge_rejects_a_key_without_a_value() {
        assert!(matches!(
            merge_parameters("kernel.module_blacklist"),
            Err(Error::MissingValue { .. })
        ));
    }

    #[test]
    fn test_merge_rejects_a_quote_in_a_list_value() {
        assert!(matches!(
            merge_parameters("kernel.module_blacklist = 'i8042\"'"),
            Err(Error::UnsupportedQuote { name }) if name == "kernel.module_blacklist"
        ));
    }
}
