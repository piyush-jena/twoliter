//! Split a kernel command line the way the kernel's `next_arg` (lib/cmdline.c) does.

/// Whitespace as the kernel's `isspace` sees it.
fn is_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

/// Split the first argument off `args` into its parameter, its value, and the rest.
fn next_arg(args: &str) -> (&str, Option<&str>, &str) {
    let (args, quoted) = match args.strip_prefix('"') {
        Some(unquoted) => (unquoted, true),
        None => (args, false),
    };
    let mut in_quote = quoted;
    let mut equals = None;
    let mut end = args.len();
    for (index, character) in args.char_indices() {
        if is_space(character) && !in_quote {
            end = index;
            break;
        }
        // Like the kernel, an `=` in the first position does not end the parameter.
        if equals.is_none() && character == '=' && index > 0 {
            equals = Some(index);
        }
        if character == '"' {
            in_quote = !in_quote;
        }
    }

    let token = &args[..end];
    let closing_quote = token.ends_with('"');
    let (param, value) = match equals {
        Some(equals) => (&token[..equals], Some(&token[equals + 1..])),
        None => (token, None),
    };
    let value = value.map(|value| match value.strip_prefix('"') {
        Some(unquoted) if closing_quote => unquoted.strip_suffix('"').unwrap_or(unquoted),
        Some(unquoted) => unquoted,
        None if quoted && closing_quote => value.strip_suffix('"').unwrap_or(value),
        None => value,
    });
    let param = match value {
        None if quoted && closing_quote => param.strip_suffix('"').unwrap_or(param),
        _ => param,
    };
    (param, value, args[end..].trim_start_matches(is_space))
}

/// The names of the kernel's parameters: those before the first `--`.
pub(crate) fn kernel_parameters(cmdline: &str) -> Vec<&str> {
    let mut names = Vec::new();
    let mut args = cmdline.trim_start_matches(is_space);
    while !args.is_empty() {
        let (param, value, rest) = next_arg(args);
        if value.is_none() && param == "--" {
            break;
        }
        names.push(param);
        args = rest;
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("key=value", "key", Some("value") ; "plain")]
    #[test_case("key=\"a b\"", "key", Some("a b") ; "quoted_value")]
    #[test_case("\"key=a b\"", "key", Some("a b") ; "quoted_argument")]
    #[test_case("flag", "flag", None ; "flag")]
    #[test_case("\"flag\"", "flag", None ; "quoted_flag")]
    #[test_case("=a=b", "=a", Some("b") ; "a_leading_equals_belongs_to_the_parameter")]
    #[test_case("key=", "key", Some("") ; "empty_value")]
    #[test_case("key=\"", "key", Some("") ; "lone_quote")]
    fn arguments_split_like_the_kernel(args: &str, param: &str, value: Option<&str>) {
        assert_eq!(next_arg(args), (param, value, ""));
    }

    #[test]
    fn quoted_spaces_stay_in_the_argument() {
        assert_eq!(
            next_arg("dm-mod.create=\"root,,,ro,0 1 verity\"  next"),
            ("dm-mod.create", Some("root,,,ro,0 1 verity"), "next")
        );
    }

    #[test]
    fn kernel_parameters_end_at_the_separator() {
        assert_eq!(
            kernel_parameters(" noinitrd\tinitrd=/a \"dm-mod.create=x y\" -- initrd=/init"),
            ["noinitrd", "initrd", "dm-mod.create"]
        );
        assert_eq!(kernel_parameters("a \"--\" b"), ["a"]);
        assert_eq!(kernel_parameters(""), Vec::<&str>::new());
    }
}
