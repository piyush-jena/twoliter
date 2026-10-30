//! Pest parser for bootconfig binary and text format.

use crate::error::{self, Result};
use crate::params::{BootconfigParams, Parameter};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use snafu::{ensure, ResultExt};

#[derive(Parser)]
#[grammar = "parsers/bootconfig.pest"]
struct BootconfigParser;

const BOOTCONFIG_MAGIC: &[u8] = b"#BOOTCONFIG\n";
const BOOTCONFIG_FIELD_SIZE: usize = size_of::<u32>();
const BOOTCONFIG_TRAILER_SIZE: usize = 2 * BOOTCONFIG_FIELD_SIZE + BOOTCONFIG_MAGIC.len();

/// Extract and validate bootconfig text from binary format.
fn extract_text(data: &[u8]) -> Result<&str> {
    ensure!(
        data.len() >= BOOTCONFIG_TRAILER_SIZE,
        error::BinaryTooShortSnafu { length: data.len() }
    );

    let (body, trailer) = data.split_at(data.len() - BOOTCONFIG_TRAILER_SIZE);
    let (size, trailer) = trailer
        .split_first_chunk::<BOOTCONFIG_FIELD_SIZE>()
        .expect("the trailer holds the size");
    let (checksum, magic) = trailer
        .split_first_chunk::<BOOTCONFIG_FIELD_SIZE>()
        .expect("the trailer holds the checksum");
    ensure!(magic == BOOTCONFIG_MAGIC, error::InvalidMagicSnafu);

    let size = usize::try_from(u32::from_le_bytes(*size)).unwrap_or(usize::MAX);
    let checksum = u32::from_le_bytes(*checksum);
    ensure!(
        size <= body.len(),
        error::InvalidBinarySizeSnafu {
            size,
            length: data.len(),
        }
    );

    let text_data = &data[..size];
    let computed = text_data
        .iter()
        .fold(0u32, |acc, b| acc.wrapping_add(u32::from(*b)));
    ensure!(
        computed == checksum,
        error::ChecksumMismatchSnafu {
            expected: checksum,
            actual: computed,
        }
    );

    // `size` includes the NUL terminator and padding.
    let text_end = text_data.iter().position(|b| *b == 0).unwrap_or(size);
    std::str::from_utf8(&text_data[..text_end]).context(error::InvalidUtf8Snafu {
        input: "bootconfig",
    })
}

/// Strip the quotes from one element of a value.
fn unquote(s: &str) -> &str {
    s.strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
        .unwrap_or(s)
}

/// One assignment, keyed by full path, borrowing its elements from the parsed text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Assignment<'a> {
    pub(crate) name: String,
    /// Each array element's own token, quotes included.
    elements: Vec<&'a str>,
}

impl Assignment<'_> {
    /// The value as written, with array elements joined by ", ".
    pub(crate) fn written(&self) -> String {
        // The kernel strips VT and FF around an unquoted value, but the grammar keeps them.
        self.elements
            .iter()
            .map(|element| element.trim_matches(['\x0b', '\x0c']))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Each array element, unquoted.
    pub(crate) fn values(&self) -> impl Iterator<Item = &str> {
        self.elements.iter().map(|element| unquote(element))
    }
}

/// Collect the assignments `pair` makes under `prefix`.
fn process_entry<'a>(
    pair: Pair<'a, Rule>,
    prefix: &str,
    assignments: &mut Vec<Assignment<'a>>,
) -> Result<()> {
    match pair.as_rule() {
        Rule::pair => {
            let mut inner = pair.into_inner();
            let key = inner.next().map(|p| p.as_str()).unwrap_or("");
            let elements = inner
                .flat_map(Pair::into_inner)
                .map(|element| element.as_str())
                .collect();
            assignments.push(Assignment {
                name: full_key(prefix, key),
                elements,
            });
        }
        Rule::bool_key => {
            let key = pair.into_inner().next().map(|p| p.as_str()).unwrap_or("");
            return error::MissingValueSnafu {
                name: full_key(prefix, key),
            }
            .fail();
        }
        Rule::block => {
            let mut inner = pair.into_inner();
            let block_name = inner.next().map(|p| p.as_str()).unwrap_or("");
            let new_prefix = full_key(prefix, block_name);
            for child in inner {
                process_entry(child, &new_prefix, assignments)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Join a key to the prefix of the block holding it.
fn full_key(prefix: &str, key: &str) -> String {
    if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{prefix}.{key}")
    }
}

/// Parse bootconfig text and extract kernel.* and init.* parameters, one per array element.
pub(crate) fn parse_text(text: &str) -> Result<BootconfigParams> {
    let mut params = BootconfigParams::default();
    for assignment in parse_assignments(text)? {
        let (parameters, name) = if let Some(name) = assignment.name.strip_prefix("kernel.") {
            (&mut params.kernel, name)
        } else if let Some(name) = assignment.name.strip_prefix("init.") {
            (&mut params.init, name)
        } else {
            continue;
        };
        parameters.extend(assignment.values().map(|value| Parameter {
            name: name.to_string(),
            value: value.to_string(),
        }));
    }
    Ok(params)
}

/// Parse bootconfig text into every assignment it makes, keyed by full path, in order.
pub(crate) fn parse_assignments(text: &str) -> Result<Vec<Assignment<'_>>> {
    let pairs = BootconfigParser::parse(Rule::config, text).context(error::BootconfigParseSnafu)?;

    let mut assignments = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::config {
            for child in pair.into_inner() {
                process_entry(child, "", &mut assignments)?;
            }
        }
    }

    Ok(assignments)
}

/// Parse bootconfig.data binary format and extract parameters.
pub(crate) fn parse(data: &[u8]) -> Result<BootconfigParams> {
    parse_text(extract_text(data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use test_case::test_case;

    use crate::test_util::encode as make_bootconfig;

    fn parameter(name: &str, value: &str) -> Parameter {
        Parameter {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    #[test]
    fn test_extract_text_valid() {
        let text = "kernel.foo = bar\ninit.baz = qux\n";
        let data = make_bootconfig(text);
        assert_eq!(extract_text(&data).unwrap(), text);
    }

    #[test_case(|d| { let i = d.len() - 1; d[i] = b'X'; } ; "invalid_magic")]
    #[test_case(|d| { let i = d.len() - 16; d[i] = 0xFF; } ; "bad_checksum")]
    #[test_case(|d| d.truncate(BOOTCONFIG_TRAILER_SIZE - 1) ; "too_short")]
    #[test_case(|d| { let i = d.len() - 20; d[i] = 0xFF; } ; "size_past_the_trailer")]
    fn test_extract_text_errors(corrupt: fn(&mut Vec<u8>)) {
        let mut data = make_bootconfig("test");
        corrupt(&mut data);
        assert!(extract_text(&data).is_err());
    }

    #[test]
    fn test_parse_text_splits_the_namespaces() {
        let params = parse_text("kernel.foo = bar\ninit.baz = qux\nother.x = 1").unwrap();
        assert_eq!(params.kernel, vec![parameter("foo", "bar")]);
        assert_eq!(params.init, vec![parameter("baz", "qux")]);
    }

    #[test]
    fn test_parse_text_array() {
        // The kernel emits `mods=a mods=b mods=c`, one argument per element.
        let params = parse_text("kernel.mods = a, b, c").unwrap();
        assert_eq!(
            params.kernel,
            vec![
                parameter("mods", "a"),
                parameter("mods", "b"),
                parameter("mods", "c"),
            ]
        );
    }

    #[test]
    fn test_parse_text_nested_block() {
        let input = "kernel {\n  sub {\n    foo = bar\n  }\n}";
        let params = parse_text(input).unwrap();
        assert_eq!(params.kernel, vec![parameter("sub.foo", "bar")]);
    }

    #[test]
    fn test_parse_text_empty_block() {
        // The image builder generates this when no package contributed a drop-in.
        let params = parse_text("kernel {}\ninit {}").unwrap();
        assert!(params.kernel.is_empty() && params.init.is_empty());
    }

    #[test_case("init.splash" ; "bool_key")]
    #[test_case("init {\n  splash\n}" ; "bool_key_in_block")]
    fn test_parse_text_rejects_bool_key(input: &str) {
        assert!(matches!(
            parse_text(input).unwrap_err(),
            Error::MissingValue { name } if name == "init.splash"
        ));
    }

    #[test]
    fn test_parse_text_syntax_error_keeps_the_diagnostic() {
        let error = parse_text("kernel.x = \"unterminated").unwrap_err();
        assert!(matches!(error, Error::BootconfigParse { .. }));
        assert!(error.to_string().contains("kernel.x"));
    }

    #[test]
    fn test_parse_assignments_keeps_every_namespace_and_assignment() {
        let input = "kernel.x = 1\nother.y = 2\nkernel.x = 3";
        let assignments = parse_assignments(input).unwrap();
        let values: Vec<(&str, &str)> = assignments
            .iter()
            .flat_map(|assignment| {
                let name = assignment.name.as_str();
                assignment.values().map(move |value| (name, value))
            })
            .collect();
        assert_eq!(
            values,
            [("kernel.x", "1"), ("other.y", "2"), ("kernel.x", "3")]
        );
    }

    #[test_case("kernel.x = 1 # comment", "kernel.x", "1", &["1"] ; "a_trailing_comment")]
    #[test_case("kernel.x = a , 'b c';", "kernel.x", "a, 'b c'", &["a", "b c"] ; "an_array")]
    #[test_case("kernel.x = \"a,b\"", "kernel.x", "\"a,b\"", &["a,b"] ; "a_quoted_comma")]
    #[test_case("kernel { x = 1 }", "kernel.x", "1", &["1"] ; "a_block")]
    fn test_parse_assignments_keeps_values_as_written(
        input: &str,
        name: &str,
        written: &str,
        values: &[&str],
    ) {
        let assignments = parse_assignments(input).unwrap();
        let [assignment] = assignments.as_slice() else {
            panic!("expected one assignment, got {assignments:?}");
        };
        assert_eq!(assignment.name, name);
        assert_eq!(assignment.written(), written);
        assert_eq!(assignment.values().collect::<Vec<_>>(), values);
    }
}
