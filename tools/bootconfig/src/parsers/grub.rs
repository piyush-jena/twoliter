//! Pest parser for grub.cfg files.

use crate::error::{self, Result};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use snafu::{OptionExt, ResultExt};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parsers/grub.pest"]
struct GrubParser;

use Rule as GrubRule;

/// A grub.cfg's statements, with `$variables` resolved in document order.
#[derive(Debug)]
pub(crate) struct GrubConfig {
    statements: Vec<Statement>,
}

/// A command or a menu entry.
#[derive(Debug)]
enum Statement {
    Command(Command),
    MenuEntry(MenuEntry),
}

/// A menu entry and the statements it runs.
#[derive(Debug)]
struct MenuEntry {
    /// Kept for fidelity; which entry boots does not depend on it yet.
    #[allow(dead_code)]
    title: String,
    /// Kept for fidelity; which entry boots does not depend on them yet.
    #[allow(dead_code)]
    options: Vec<String>,
    body: Vec<Statement>,
}

/// A command and its resolved arguments.
#[derive(Debug)]
struct Command {
    name: String,
    arguments: Vec<String>,
}

impl GrubConfig {
    /// Parse grub.cfg, applying each `set` to everything that follows it.
    pub(crate) fn parse(grub_cfg: &[u8]) -> Result<Self> {
        let text =
            std::str::from_utf8(grub_cfg).context(error::InvalidUtf8Snafu { input: "grub.cfg" })?;
        let pairs = GrubParser::parse(GrubRule::config, text).context(error::GrubParseSnafu)?;
        let mut variables = HashMap::new();
        let statements = pairs
            .flat_map(Pair::into_inner)
            .flat_map(|pair| statement(pair, &mut variables))
            .collect();
        Ok(Self { statements })
    }

    /// Arguments of the `linux` command that boots: the last one in document order.
    pub(crate) fn linux_arguments(&self) -> Result<&[String]> {
        last_linux(&self.statements)
            .map(|command| command.arguments.as_slice())
            .context(error::MissingGrubLinuxDirectiveSnafu)
    }
}

/// The last `linux` command among `statements` and the entries nested in them.
fn last_linux(statements: &[Statement]) -> Option<&Command> {
    statements
        .iter()
        .rev()
        .find_map(|statement| match statement {
            Statement::Command(command) => (command.name == "linux").then_some(command),
            Statement::MenuEntry(entry) => last_linux(&entry.body),
        })
}

/// Build the statement a pair holds, applying it to `variables` if it is a `set`.
fn statement(
    pair: Pair<'_, GrubRule>,
    variables: &mut HashMap<String, String>,
) -> Option<Statement> {
    match pair.as_rule() {
        GrubRule::set_stmt => {
            let mut inner = pair.into_inner();
            if let (Some(name), Some(value)) = (inner.next(), inner.next()) {
                let resolved = resolve_set_value(value, variables);
                variables.insert(name.as_str().to_string(), resolved);
            }
            None
        }
        GrubRule::command => {
            let mut inner = pair.into_inner();
            let name = inner.next()?.as_str().to_string();
            let arguments = inner
                .map(|argument| resolve_pair(argument, variables))
                .collect();
            Some(Statement::Command(Command { name, arguments }))
        }
        GrubRule::menuentry => {
            let mut inner = pair.into_inner();
            let title = inner.next().map(|title| resolve_quoted(title, variables))?;
            let (blocks, options): (Vec<_>, Vec<_>) =
                inner.partition(|pair| pair.as_rule() == GrubRule::block);
            let options = options
                .iter()
                .map(|option| option.as_str().to_string())
                .collect();
            let body = blocks
                .into_iter()
                .flat_map(|block| statements(block.into_inner(), variables))
                .collect();
            Some(Statement::MenuEntry(MenuEntry {
                title,
                options,
                body,
            }))
        }
        _ => None,
    }
}

/// Build the statements among `pairs`, in order.
fn statements(
    pairs: Pairs<'_, GrubRule>,
    variables: &mut HashMap<String, String>,
) -> Vec<Statement> {
    pairs
        .filter_map(|pair| statement(pair, variables))
        .collect()
}

/// Resolve a quoted_string, variable, keyvalue or compound_word pair, expanding variables.
fn resolve_pair(pair: Pair<'_, GrubRule>, variables: &HashMap<String, String>) -> String {
    match pair.as_rule() {
        GrubRule::variable => {
            let var_name = pair
                .into_inner()
                .find_map(|p| {
                    if p.as_rule() == GrubRule::var_name {
                        Some(p.as_str())
                    } else {
                        p.into_inner()
                            .find(|inner| inner.as_rule() == GrubRule::var_name)
                            .map(|inner| inner.as_str())
                    }
                })
                .unwrap_or("");
            variables.get(var_name).cloned().unwrap_or_default()
        }
        GrubRule::quoted_string => format!("\"{}\"", resolve_quoted(pair, variables)),
        GrubRule::keyvalue => {
            let mut result = String::new();
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    GrubRule::key_part => {
                        result.push_str(inner.as_str());
                        result.push('=');
                    }
                    GrubRule::quoted_string => result.push_str(&resolve_pair(inner, variables)),
                    GrubRule::value_part => result.push_str(inner.as_str()),
                    _ => {}
                }
            }
            result
        }
        GrubRule::compound_word => {
            let mut result = String::new();
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    GrubRule::variable => result.push_str(&resolve_pair(inner, variables)),
                    GrubRule::word_text => result.push_str(inner.as_str()),
                    _ => {}
                }
            }
            result
        }
        _ => pair.as_str().to_string(),
    }
}

/// Resolve the contents of a quoted_string, without its quotes.
fn resolve_quoted(pair: Pair<'_, GrubRule>, variables: &HashMap<String, String>) -> String {
    pair.into_inner()
        .map(|inner| match inner.as_rule() {
            GrubRule::variable => resolve_pair(inner, variables),
            GrubRule::string_text => inner.as_str().to_string(),
            _ => String::new(),
        })
        .collect()
}

/// Resolve a set statement value, which may be a quoted string with variables.
fn resolve_set_value(pair: Pair<'_, GrubRule>, variables: &HashMap<String, String>) -> String {
    match pair.as_rule() {
        GrubRule::quoted_string => resolve_quoted(pair, variables),
        _ => pair.as_str().to_string(),
    }
}

/// The parser as it was before `GrubConfig`, kept as an oracle for it.
#[cfg(test)]
mod legacy {
    use super::{GrubParser, GrubRule};
    use crate::error::{self, Result};
    use pest::iterators::Pair;
    use pest::Parser;
    use snafu::{OptionExt, ResultExt};
    use std::collections::HashMap;

    /// Resolve a quoted_string or variable pair, expanding variables from the map.
    fn resolve_pair(pair: Pair<'_, GrubRule>, variables: &HashMap<String, String>) -> String {
        match pair.as_rule() {
            GrubRule::variable => {
                let var_name = pair
                    .into_inner()
                    .find_map(|p| {
                        if p.as_rule() == GrubRule::var_name {
                            Some(p.as_str())
                        } else {
                            p.into_inner()
                                .find(|inner| inner.as_rule() == GrubRule::var_name)
                                .map(|inner| inner.as_str())
                        }
                    })
                    .unwrap_or("");
                variables.get(var_name).cloned().unwrap_or_default()
            }
            GrubRule::quoted_string => {
                let mut result = String::from("\"");
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        GrubRule::variable => result.push_str(&resolve_pair(inner, variables)),
                        GrubRule::string_text => result.push_str(inner.as_str()),
                        _ => {}
                    }
                }
                result.push('"');
                result
            }
            GrubRule::keyvalue => {
                let mut result = String::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        GrubRule::key_part => {
                            result.push_str(inner.as_str());
                            result.push('=');
                        }
                        GrubRule::quoted_string => result.push_str(&resolve_pair(inner, variables)),
                        GrubRule::value_part => result.push_str(inner.as_str()),
                        _ => {}
                    }
                }
                result
            }
            GrubRule::compound_word => {
                let mut result = String::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        GrubRule::variable => result.push_str(&resolve_pair(inner, variables)),
                        GrubRule::word_text => result.push_str(inner.as_str()),
                        _ => {}
                    }
                }
                result
            }
            _ => pair.as_str().to_string(),
        }
    }

    /// Resolve a set statement value, which may be a quoted string with variables.
    fn resolve_set_value(pair: Pair<'_, GrubRule>, variables: &HashMap<String, String>) -> String {
        match pair.as_rule() {
            GrubRule::quoted_string => {
                let mut result = String::new();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        GrubRule::variable => result.push_str(&resolve_pair(inner, variables)),
                        GrubRule::string_text => result.push_str(inner.as_str()),
                        _ => {}
                    }
                }
                result
            }
            GrubRule::unquoted_set_value => pair.as_str().to_string(),
            _ => pair.as_str().to_string(),
        }
    }

    /// Parse grub.cfg and extract the kernel command line with variables resolved.
    pub(super) fn parse(grub_cfg: &[u8]) -> Result<Vec<String>> {
        let text =
            std::str::from_utf8(grub_cfg).context(error::InvalidUtf8Snafu { input: "grub.cfg" })?;

        let pairs = GrubParser::parse(GrubRule::config, text).context(error::GrubParseSnafu)?;

        let mut variables: HashMap<String, String> = HashMap::new();
        let mut directives: Vec<Vec<String>> = Vec::new();

        for pair in pairs.flatten() {
            match pair.as_rule() {
                GrubRule::set_stmt => {
                    let mut inner = pair.into_inner();
                    if let Some(name) = inner.next() {
                        if let Some(value) = inner.next() {
                            let resolved = resolve_set_value(value, &variables);
                            variables.insert(name.as_str().to_string(), resolved);
                        }
                    }
                }
                GrubRule::command => {
                    let mut inner = pair.into_inner();
                    if let Some(cmd) = inner.next() {
                        if cmd.as_str() == "linux" {
                            directives
                                .push(inner.map(|arg| resolve_pair(arg, &variables)).collect());
                        }
                    }
                }
                _ => {}
            }
        }

        directives
            .pop()
            .context(error::MissingGrubLinuxDirectiveSnafu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    /// Join the arguments of the `linux` directive, so that a case can state
    /// the command line GRUB would pass on.
    fn linux_args(grub_cfg: &[u8]) -> String {
        GrubConfig::parse(grub_cfg)
            .unwrap()
            .linux_arguments()
            .unwrap()
            .join(" ")
    }

    #[test]
    fn test_parse_full_config() {
        let grub_cfg = br#"
set default="0"
set dm_verity_root="root,,,ro,0 123"

menuentry "test" {
   linux ($root)/vmlinuz \
       console=tty0 \
       dm-mod.create="$dm_verity_root" \
       -- \
       systemd.log_target=journal
   initrd ($private)/bootconfig.data
}
"#;
        let cmdline = linux_args(grub_cfg);
        assert!(cmdline.starts_with("()/vmlinuz"));
        assert!(cmdline.contains("console=tty0"));
        assert!(cmdline.contains("dm-mod.create=\"root,,,ro,0 123\""));
        assert!(cmdline.contains("-- systemd.log_target=journal"));
    }

    // Variable resolution tests
    #[test_case("set foo=bar\nlinux /vmlinuz $foo", "/vmlinuz bar" ; "bare_variable")]
    #[test_case("set foo=bar\nlinux /vmlinuz ${foo}", "/vmlinuz bar" ; "braced_variable")]
    #[test_case("linux /vmlinuz $unknown", "/vmlinuz " ; "unknown_variable")]
    #[test_case("set foo=\"bar\"\nset baz=\"I am a ${foo}\"\nlinux /vmlinuz $baz", "/vmlinuz I am a bar" ; "nested_variable")]
    #[test_case("set a=1\nset b=2\nlinux /vmlinuz $a $b", "/vmlinuz 1 2" ; "multiple_variables")]
    fn test_variable_resolution(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Set statement tests
    #[test_case("set foo=bar\nlinux /vmlinuz $foo", "/vmlinuz bar" ; "unquoted_value")]
    #[test_case("set foo=\"bar\"\nlinux /vmlinuz $foo", "/vmlinuz bar" ; "quoted_value")]
    #[test_case("set foo=\"bar baz\"\nlinux /vmlinuz $foo", "/vmlinuz bar baz" ; "quoted_with_spaces")]
    fn test_set_statement(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Linux command argument tests
    #[test_case("linux /vmlinuz console=tty0", "/vmlinuz console=tty0" ; "simple_arg")]
    #[test_case("linux /vmlinuz a b c", "/vmlinuz a b c" ; "multiple_args")]
    #[test_case("linux /vmlinuz key=\"value\"", "/vmlinuz key=\"value\"" ; "quoted_value_arg")]
    #[test_case("linux /vmlinuz --", "/vmlinuz --" ; "separator")]
    #[test_case("linux /vmlinuz a -- b", "/vmlinuz a -- b" ; "args_with_separator")]
    fn test_linux_args(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Line continuation tests
    #[test_case("linux /vmlinuz \\\n  a \\\n  b", "/vmlinuz a b" ; "line_continuation")]
    #[test_case("set foo=bar\nlinux /vmlinuz \\\n  $foo", "/vmlinuz bar" ; "continuation_with_var")]
    fn test_line_continuation(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Comment tests
    #[test_case("# comment\nlinux /vmlinuz a", "/vmlinuz a" ; "comment_line")]
    #[test_case("linux /vmlinuz a # inline comment\n", "/vmlinuz a" ; "inline_comment")]
    fn test_comments(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Menuentry tests
    #[test_case("menuentry \"test\" {\n  linux /vmlinuz a\n}", "/vmlinuz a" ; "simple_menuentry")]
    #[test_case("menuentry \"test\" --unrestricted {\n  linux /vmlinuz a\n}", "/vmlinuz a" ; "menuentry_with_option")]
    fn test_menuentry(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    // Kernel path with GRUB device syntax
    #[test_case("linux ($root)/vmlinuz a", "()/vmlinuz a" ; "grub_device_unset")]
    #[test_case("set root=hd0,gpt3\nlinux ($root)/vmlinuz a", "(hd0,gpt3)/vmlinuz a" ; "grub_device_set")]
    fn test_kernel_path(input: &str, expected: &str) {
        assert_eq!(linux_args(input.as_bytes()), expected);
    }

    /// Every input either parser accepts gives the same arguments, and every other input
    /// the same error.
    #[test]
    fn matches_the_legacy_parser() {
        let configs: &[&[u8]] = &[
            b"set foo=bar\nlinux /vmlinuz $foo ${foo} $unknown",
            b"set foo=\"bar\"\nset baz=\"I am a ${foo}\"\nlinux /vmlinuz $baz",
            b"set foo=\"bar baz\"\nlinux /vmlinuz key=\"value\" a -- b \\\n  $foo # comment\n",
            b"menuentry \"test\" --unrestricted {\n  linux ($root)/vmlinuz a\n}",
            b"set root=hd0,gpt3\nlinux ($root)/vmlinuz a",
            // Several `linux` lines, in and out of entries: the last in document order boots.
            b"menuentry \"a\" {\n  linux /a\n}\nmenuentry \"b\" {\n  linux /b\n}",
            b"menuentry \"a\" {\n  linux /a\n}\nlinux /top",
            b"linux /top\nmenuentry \"a\" {\n  linux /a\n  linux /a2\n}",
            b"menuentry \"outer\" {\n  menuentry \"inner\" {\n    linux /inner\n  }\n  linux /outer\n}",
            // A `set` applies to everything after it, whichever entry it is in.
            b"menuentry \"a\" {\n  set v=1\n  linux /a\n}\nmenuentry \"b\" {\n  linux /b $v\n}",
            b"menuentry \"a\" {\n  linux /a $later\n}\nset later=2",
            b"if true; then\n  linux /a\nelse\n  linux /b\nfi",
            br#"set default="0"
set dm_verity_root="root,,,ro,0 123 verity PARTUUID=$boot_uuid/PARTNROFF=1"
menuentry "Bottlerocket OS 1.67.0" --unrestricted {
   linux ($root)/vmlinuz \
       console=tty0 \
       bootconfig \
       dm-mod.create="$dm_verity_root" \
       -- \
       systemd.log_target=journal-or-kmsg
   initrd ($private)/bootconfig.data
   boot
}
"#,
            b"set only=1\n",
            b"menuentry {",
            b"linux /vmlinuz \xff",
        ];
        for config in configs {
            let new = GrubConfig::parse(config)
                .and_then(|config| config.linux_arguments().map(<[String]>::to_vec));
            match (new, legacy::parse(config)) {
                (Ok(new), Ok(old)) => assert_eq!(new, old),
                (Err(new), Err(old)) => assert_eq!(new.to_string(), old.to_string()),
                (new, old) => panic!(
                    "{:?}: new {new:?}, old {old:?}",
                    String::from_utf8_lossy(config)
                ),
            }
        }
    }

    #[test_case(b"menuentry \"a\" {\n  linux /a\n}\nmenuentry \"b\" {\n  linux /b\n}", "/b" ; "the_last_entry")]
    #[test_case(b"menuentry \"a\" {\n  linux /a\n}\nlinux /top", "/top" ; "a_top_level_line_after_an_entry")]
    #[test_case(b"menuentry \"a\" {\n  linux /a\n  linux /a2\n}", "/a2" ; "the_last_line_of_an_entry")]
    fn the_last_linux_in_document_order_boots(input: &[u8], expected: &str) {
        assert_eq!(linux_args(input), expected);
    }
}
