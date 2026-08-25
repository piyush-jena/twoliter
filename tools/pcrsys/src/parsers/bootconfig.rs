//! Pest parser for bootconfig binary and text format.

use crate::error::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use snafu::{whatever, ResultExt};

/// A single bootconfig entry: a fully-composed key plus its ordered
/// list of values.
///
/// An **empty** `values` vector denotes a *value-less* key (rendered by
/// `xbc_snprint_cmdline` as `key ` with no `=`). A key with one or more
/// values is either a scalar (one value) or an array (multiple values);
/// arrays are preserved element-by-element so the renderer can emit one
/// repeated `key=value ` token per element, exactly as the kernel's
/// `xbc_snprint_cmdline()` does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootEntry {
    /// Fully-composed key with the `kernel.`/`init.` prefix stripped and
    /// any nested block names joined with `.`.
    pub key: String,
    /// Ordered values for the key. Empty = value-less key.
    pub values: Vec<String>,
}

/// Parsed bootconfig parameters split by prefix.
#[derive(Debug, Default)]
pub struct BootconfigParams {
    /// kernel.* parameters (prefix stripped).
    pub kernel: Vec<BootEntry>,
    /// init.* parameters (prefix stripped).
    pub init: Vec<BootEntry>,
}

#[derive(Parser)]
#[grammar = "parsers/bootconfig.pest"]
struct BootconfigParser;

use Rule as BootconfigRule;

const BOOTCONFIG_MAGIC: &[u8] = b"#BOOTCONFIG\n";

/// Extract and validate bootconfig text from binary format.
///
/// Format: `[text][padding][size:u32 LE][checksum:u32 LE][magic:12 bytes]`
fn extract_text(data: &[u8]) -> Result<&str> {
    if data.len() < 20 {
        whatever!("bootconfig data too short");
    }

    let magic_start = data.len() - 12;
    if &data[magic_start..] != BOOTCONFIG_MAGIC {
        whatever!("invalid bootconfig magic");
    }

    let size_start = magic_start - 8;
    let size = u32::from_le_bytes(
        data[size_start..size_start + 4]
            .try_into()
            .whatever_context("failed to read size")?,
    ) as usize;

    let checksum = u32::from_le_bytes(
        data[size_start + 4..size_start + 8]
            .try_into()
            .whatever_context("failed to read checksum")?,
    );

    if size > size_start {
        whatever!("bootconfig size {} exceeds data length", size);
    }

    let text_data = &data[..size];
    let computed: u32 = text_data
        .iter()
        .fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
    if computed != checksum {
        whatever!(
            "bootconfig checksum mismatch: expected {}, got {}",
            checksum,
            computed
        );
    }

    // Find the actual text end (before any null padding that might be included in size)
    let text_end = text_data.iter().position(|&b| b == 0).unwrap_or(size);
    std::str::from_utf8(&text_data[..text_end])
        .whatever_context("bootconfig text is not valid UTF-8")
}

/// Extract value elements, handling quoted strings and arrays.
///
/// Returns one `String` per array element (a scalar yields a single
/// element). Quotes are stripped from each individual element; array
/// elements are **not** joined so the renderer can faithfully emit one
/// repeated `key=value ` token per element.
fn extract_value(pair: Pair<'_, BootconfigRule>) -> Vec<String> {
    let mut values = Vec::new();
    for item in pair.into_inner() {
        let s = item.as_str();
        // Strip quotes from strings
        let v = s
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(s);
        values.push(v.to_string());
    }
    values
}

/// Process a pair or bool_key, adding to params with the given prefix.
fn process_entry(pair: Pair<'_, BootconfigRule>, prefix: &str, params: &mut BootconfigParams) {
    match pair.as_rule() {
        BootconfigRule::pair => {
            let mut inner = pair.into_inner();
            let key = inner.next().map(|p| p.as_str()).unwrap_or("");
            let full_key = if prefix.is_empty() {
                key.to_string()
            } else {
                format!("{prefix}.{key}")
            };
            let value = inner.next().map(extract_value).unwrap_or_default();

            if let Some(rest) = full_key.strip_prefix("kernel.") {
                params.kernel.push(BootEntry {
                    key: rest.to_string(),
                    values: value,
                });
            } else if let Some(rest) = full_key.strip_prefix("init.") {
                params.init.push(BootEntry {
                    key: rest.to_string(),
                    values: value,
                });
            }
        }
        BootconfigRule::bool_key => {
            let key = pair.into_inner().next().map(|p| p.as_str()).unwrap_or("");
            let full_key = if prefix.is_empty() {
                key.to_string()
            } else {
                format!("{prefix}.{key}")
            };

            if let Some(rest) = full_key.strip_prefix("kernel.") {
                params.kernel.push(BootEntry {
                    key: rest.to_string(),
                    values: Vec::new(),
                });
            } else if let Some(rest) = full_key.strip_prefix("init.") {
                params.init.push(BootEntry {
                    key: rest.to_string(),
                    values: Vec::new(),
                });
            }
        }
        BootconfigRule::block => {
            let mut inner = pair.into_inner();
            let block_name = inner.next().map(|p| p.as_str()).unwrap_or("");
            let new_prefix = if prefix.is_empty() {
                block_name.to_string()
            } else {
                format!("{prefix}.{block_name}")
            };
            for child in inner {
                process_entry(child, &new_prefix, params);
            }
        }
        _ => {}
    }
}

/// Parse bootconfig text and extract kernel.* and init.* parameters.
fn parse_text(text: &str) -> Result<BootconfigParams> {
    let pairs = BootconfigParser::parse(BootconfigRule::config, text)
        .whatever_context("failed to parse bootconfig")?;

    let mut params = BootconfigParams::default();

    for pair in pairs {
        if pair.as_rule() == BootconfigRule::config {
            for child in pair.into_inner() {
                process_entry(child, "", &mut params);
            }
        }
    }

    Ok(params)
}

/// Parse bootconfig.data binary format and extract parameters.
pub fn parse(data: &[u8]) -> Result<BootconfigParams> {
    let text = extract_text(data)?;
    parse_text(text)
}

/// Return `true` iff `val` contains one of the ASCII characters the
/// kernel tests for with `strpbrk(val, " \t\r\n")`.
///
/// This is deliberately **not** Rust's Unicode-aware
/// `char::is_whitespace` (constraint C2): only the four ASCII bytes
/// space, tab, carriage-return and line-feed trigger quoting, so a
/// value containing e.g. U+00A0 (non-breaking space) is left unquoted,
/// exactly as `lib/bootconfig.c` does.
fn needs_quote(val: &str) -> bool {
    val.bytes()
        .any(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
}

/// Faithful Rust port of the Linux kernel's `xbc_snprint_cmdline()`
/// (`lib/bootconfig.c`).
///
/// The kernel walks every leaf key in order and, for each, composes the
/// full dotted key (`xbc_node_compose_key_after`) and then:
///
///   * for a **value-less** key (empty `values`), emits `key ` — the key
///     followed by a single space, with **no** `=`;
///   * otherwise iterates the key's values (`xbc_array_for_each_value`)
///     and, for each value, emits `key=<q>value<q> ` via
///     `snprintf("%s=%s%s%s ", key, q, val, q)`, where `<q>` is a double
///     quote iff `strpbrk(val, " \t\r\n")` is non-NULL (see
///     [`needs_quote`]) and empty otherwise. Every token is terminated
///     by a single trailing space.
///
/// An array value therefore renders as repeated `key=value ` tokens —
/// one per element — never a comma-joined single token.
///
/// `BootEntry::key` is expected to already be fully composed (nested
/// block names joined with `.`), which the parser handles when building
/// the entries, so this function mirrors the kernel loop directly.
pub fn xbc_snprint_cmdline(entries: &[BootEntry]) -> String {
    let mut out = String::new();
    for entry in entries {
        if entry.values.is_empty() {
            // Value-less key: `key ` (no `=`).
            out.push_str(&entry.key);
            out.push(' ');
        } else {
            // One `key=<q>val<q> ` token per value (arrays repeat the key).
            for val in &entry.values {
                let q = if needs_quote(val) { "\"" } else { "" };
                out.push_str(&entry.key);
                out.push('=');
                out.push_str(q);
                out.push_str(val);
                out.push_str(q);
                out.push(' ');
            }
        }
    }
    out
}

/// Format bootconfig parameters as a kernel command line fragment.
///
/// Thin wrapper around the [`xbc_snprint_cmdline`] port so existing call
/// sites (`pcr9.rs`) keep a stable name while the rendering logic lives
/// in one faithful kernel-mirroring function.
pub fn format_params(params: &[BootEntry]) -> String {
    xbc_snprint_cmdline(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn make_bootconfig(text: &str) -> Vec<u8> {
        let text_bytes = text.as_bytes();
        let size = text_bytes.len() as u32;
        let checksum: u32 = text_bytes.iter().map(|&b| b as u32).sum();

        let padding = (4 - (text_bytes.len() % 4)) % 4;
        let mut data = text_bytes.to_vec();
        data.extend(vec![0u8; padding]);
        data.extend(size.to_le_bytes());
        data.extend(checksum.to_le_bytes());
        data.extend(BOOTCONFIG_MAGIC);
        data
    }

    #[test]
    fn test_extract_text_valid() {
        let text = "kernel.foo = bar\ninit.baz = qux\n";
        let data = make_bootconfig(text);
        assert_eq!(extract_text(&data).unwrap(), text);
    }

    #[test]
    fn test_extract_text_with_null_padding() {
        // Simulate bootconfig where size includes null padding bytes
        let text = "kernel.foo = bar\n";
        let text_bytes = text.as_bytes();
        let padding = 3; // pad to 4-byte alignment
        let size_with_padding = (text_bytes.len() + padding) as u32;
        let checksum: u32 = text_bytes.iter().map(|&b| b as u32).sum();

        let mut data = text_bytes.to_vec();
        data.extend(vec![0u8; padding]);
        data.extend(size_with_padding.to_le_bytes());
        data.extend(checksum.to_le_bytes());
        data.extend(BOOTCONFIG_MAGIC);

        // Should extract just the text without null bytes
        assert_eq!(extract_text(&data).unwrap(), text);
    }

    #[test_case(|d| { let i = d.len() - 1; d[i] = b'X'; } ; "invalid_magic")]
    #[test_case(|d| { let i = d.len() - 16; d[i] = 0xFF; } ; "bad_checksum")]
    fn test_extract_text_errors(corrupt: fn(&mut Vec<u8>)) {
        let mut data = make_bootconfig("test");
        corrupt(&mut data);
        assert!(extract_text(&data).is_err());
    }

    #[test_case("kernel.FOO = bar", "FOO", "bar" ; "unquoted")]
    #[test_case("kernel.FOO = \"bar\"", "FOO", "bar" ; "double_quoted")]
    #[test_case("kernel.FOO = 'bar'", "FOO", "bar" ; "single_quoted")]
    #[test_case("kernel.MULTI = \"with spaces\"", "MULTI", "with spaces" ; "with_spaces")]
    #[test_case("kernel.FOO = bar;", "FOO", "bar" ; "semicolon_terminated")]
    fn test_parse_text_kernel(input: &str, key: &str, value: &str) {
        let params = parse_text(input).unwrap();
        assert_eq!(params.kernel, vec![entry(key, &[value])]);
    }

    #[test_case("init.BAZ = qux", "BAZ", "qux" ; "init_simple")]
    fn test_parse_text_init(input: &str, key: &str, value: &str) {
        let params = parse_text(input).unwrap();
        assert_eq!(params.init, vec![entry(key, &[value])]);
    }

    #[test]
    fn test_parse_text_array() {
        let params = parse_text("kernel.mods = a, b, c").unwrap();
        assert_eq!(params.kernel, vec![entry("mods", &["a", "b", "c"])]);
    }

    #[test]
    fn test_parse_text_block() {
        let input = "kernel {\n  foo = bar\n  baz = qux\n}";
        let params = parse_text(input).unwrap();
        assert_eq!(params.kernel.len(), 2);
        assert_eq!(params.kernel[0], entry("foo", &["bar"]));
        assert_eq!(params.kernel[1], entry("baz", &["qux"]));
    }

    #[test]
    fn test_parse_text_nested_block() {
        let input = "kernel {\n  sub {\n    foo = bar\n  }\n}";
        let params = parse_text(input).unwrap();
        assert_eq!(params.kernel, vec![entry("sub.foo", &["bar"])]);
    }

    #[test]
    fn test_parse_text_bool_key() {
        let params = parse_text("init.splash").unwrap();
        assert_eq!(params.init, vec![entry("splash", &[])]);
    }

    #[test]
    fn test_parse_text_bool_key_in_block() {
        let input = "init {\n  splash\n  quiet\n}";
        let params = parse_text(input).unwrap();
        assert_eq!(params.init.len(), 2);
        assert_eq!(params.init[0], entry("splash", &[]));
        assert_eq!(params.init[1], entry("quiet", &[]));
    }

    #[test]
    fn test_parse_text_ignores_comments() {
        let params = parse_text("# comment\nkernel.FOO = bar").unwrap();
        assert_eq!(params.kernel.len(), 1);
    }

    #[test]
    fn test_parse_text_kernel_docs_example() {
        // Example from kernel docs
        let input = r#"
kernel {
    root = 01234567-89ab-cdef-0123-456789abcd
}
init {
    splash
}
"#;
        let params = parse_text(input).unwrap();
        assert_eq!(
            params.kernel,
            vec![entry("root", &["01234567-89ab-cdef-0123-456789abcd"])]
        );
        assert_eq!(params.init, vec![entry("splash", &[])]);
    }

    #[test_case(&[("FOO", "bar")], "FOO=bar " ; "simple")]
    #[test_case(&[("MULTI", "with spaces")], "MULTI=\"with spaces\" " ; "quoted")]
    #[test_case(&[("A", "1"), ("B", "2")], "A=1 B=2 " ; "multiple")]
    fn test_format_params(input: &[(&str, &str)], expected: &str) {
        let params: Vec<_> = input.iter().map(|(k, v)| entry(k, &[v])).collect();
        assert_eq!(format_params(&params), expected);
    }

    // ------------------------------------------------------------------
    // T01: xbc_snprint_cmdline() kernel-reference vectors.
    //
    // These encode the exact rules of the Linux kernel's
    // xbc_snprint_cmdline() (lib/bootconfig.c) and target the corrected
    // renderer + value model landing in T02-T03. They are expected to
    // FAIL (against the not-yet-existing `BootEntry` model and
    // `xbc_snprint_cmdline()` function) until those tasks are complete.
    //
    // Kernel rules (see spec Constraints C1/C2):
    //   * value-less key            -> "key "         (no '=')
    //   * scalar w/o ` \t\r\n`      -> "key=val "     (unquoted)
    //   * scalar with ` \t\r\n`     -> "key=\"val\" " (quoted)
    //   * array [a, b, c]           -> "key=a key=b key=c " (repeated)
    //   * quoting tests ONLY the ASCII set { space, tab, \r, \n }
    //     (NOT Unicode char::is_whitespace, so U+00A0 stays unquoted)
    // ------------------------------------------------------------------

    /// Build a `BootEntry` for the renderer tests. An empty `values`
    /// slice denotes a value-less key.
    fn entry(key: &str, values: &[&str]) -> BootEntry {
        BootEntry {
            key: key.to_string(),
            values: values.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn xbc_valueless_key_has_no_equals() {
        // AC-1: value-less key renders as "key " with no '='.
        assert_eq!(xbc_snprint_cmdline(&[entry("splash", &[])]), "splash ");
    }

    #[test]
    fn xbc_unquoted_scalar() {
        // AC-2: value without ` \t\r\n` renders unquoted.
        assert_eq!(xbc_snprint_cmdline(&[entry("FOO", &["bar"])]), "FOO=bar ");
    }

    #[test]
    fn xbc_quoted_space_value() {
        // AC-2: value containing a space is double-quoted.
        assert_eq!(
            xbc_snprint_cmdline(&[entry("MULTI", &["with spaces"])]),
            "MULTI=\"with spaces\" "
        );
    }

    #[test_case("\t" ; "tab")]
    #[test_case("\r" ; "carriage_return")]
    #[test_case("\n" ; "line_feed")]
    fn xbc_quotes_each_ascii_whitespace(ws: &str) {
        // AC-2: each of tab, CR, LF triggers quoting (strpbrk set).
        let val = format!("a{ws}b");
        assert_eq!(
            xbc_snprint_cmdline(&[entry("K", &[&val])]),
            format!("K=\"{val}\" ")
        );
    }

    #[test]
    fn xbc_array_repeats_key() {
        // AC-3: array renders as one repeated key token per element,
        // NOT a single comma-joined value.
        assert_eq!(
            xbc_snprint_cmdline(&[entry("mods", &["a", "b", "c"])]),
            "mods=a mods=b mods=c "
        );
    }

    #[test]
    fn xbc_non_ascii_space_not_quoted() {
        // AC-4: U+00A0 (non-breaking space) is NOT in { space, tab,
        // \r, \n }, so the value must NOT be quoted (matches kernel
        // strpbrk, unlike Rust's Unicode char::is_whitespace).
        assert_eq!(
            xbc_snprint_cmdline(&[entry("NB", &["a\u{00A0}b"])]),
            "NB=a\u{00A0}b "
        );
    }

    #[test]
    fn xbc_multiple_entries_and_forms() {
        // Combined: value-less + scalar + array + quoted, in order.
        let entries = [
            entry("quiet", &[]),
            entry("root", &["UUID=abc"]),
            entry("mods", &["x", "y"]),
            entry("msg", &["hello world"]),
        ];
        assert_eq!(
            xbc_snprint_cmdline(&entries),
            "quiet root=UUID=abc mods=x mods=y msg=\"hello world\" "
        );
    }
}
