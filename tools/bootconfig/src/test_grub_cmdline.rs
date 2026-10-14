use super::*;
use test_case::test_case;

#[test_case("key=value", "key=value" ; "no_quotes_unchanged")]
#[test_case("simple", "simple" ; "no_equals_unchanged")]
#[test_case(r#"key="value""#, r#""key=value""# ; "simple_quoted_value")]
#[test_case(r#"key="value with spaces""#, r#""key=value with spaces""# ; "quoted_value_with_spaces")]
#[test_case(r#"foo=bar key="quoted value" baz=qux"#, r#"foo=bar "key=quoted value" baz=qux"# ; "mixed_quoted_and_unquoted")]
#[test_case(r#"dm-mod.create="root,,,ro,0 123 verity""#, r#""dm-mod.create=root,,,ro,0 123 verity""# ; "dm_mod_create_style")]
#[test_case(r#"a="1" b="2" c="3""#, r#""a=1" "b=2" "c=3""# ; "multiple_quoted_values")]
#[test_case(r#"first="val""#, r#""first=val""# ; "quoted_at_start")]
#[test_case("", "" ; "empty_string")]
fn test_repair_quotes(input: &str, expected: &str) {
    assert_eq!(repair_quotes(input), expected);
}
