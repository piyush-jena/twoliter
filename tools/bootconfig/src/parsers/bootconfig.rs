use crate::{error, BootConfig, Parameter};
use error::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use snafu::{ensure, ResultExt};

#[derive(Parser)]
#[grammar = "parsers/bootconfig.pest"]
pub(crate) struct BootconfigParser;

// Linux traverses keys depth first, in order of first insertion. Keeping this
// tree matters when dotted keys with a common prefix are not adjacent in text.
#[derive(Default)]
struct Node {
    values: Option<Vec<Option<String>>>,
    children: Vec<(String, Node)>,
}

impl Node {
    fn insert(&mut self, key: &str, values: Option<Vec<Option<String>>>) {
        let (name, rest) = key.split_once('.').unwrap_or((key, ""));
        let index = match self.children.iter().position(|(k, _)| k == name) {
            Some(index) => index,
            None => {
                self.children.push((name.to_owned(), Node::default()));
                self.children.len() - 1
            }
        };
        let node = &mut self.children[index].1;
        if !rest.is_empty() {
            return node.insert(rest, values);
        }
        if let Some(values) = values {
            // Later assignments replace values; a repeated flag does not erase
            // an assignment already stored at this key.
            if node.values.is_none() || values != [None] {
                node.values = Some(values);
            }
        }
    }

    fn collect(&self, prefix: &str, out: &mut Vec<Parameter>) {
        if self.values.is_none() && self.children.is_empty() && !prefix.is_empty() {
            out.push(Parameter {
                name: prefix.to_owned(),
                value: None,
            });
        }
        // A flag with children is a namespace in Linux bootconfig, so only its
        // descendants are emitted. An assigned value can coexist with children.
        if let Some(values) = self
            .values
            .as_ref()
            .filter(|values| values.as_slice() != [None] || self.children.is_empty())
        {
            for value in values {
                out.push(Parameter {
                    name: prefix.to_owned(),
                    value: value.clone(),
                });
            }
        }
        for (name, child) in &self.children {
            let key = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{prefix}.{name}")
            };
            child.collect(&key, out);
        }
    }
}

fn process(pair: Pair<'_, Rule>, prefix: &str, tree: &mut Node) {
    match pair.as_rule() {
        Rule::pair | Rule::bool_key | Rule::block => {
            let rule = pair.as_rule();
            let mut inner = pair.into_inner();
            let name = inner.next().expect("grammar requires key").as_str();
            let key = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}.{name}")
            };
            match rule {
                Rule::block => {
                    tree.insert(&key, None);
                    for child in inner {
                        process(child, &key, tree);
                    }
                }
                Rule::bool_key => tree.insert(&key, Some(vec![None])),
                Rule::pair => {
                    let values = inner
                        .flat_map(Pair::into_inner)
                        .map(|value| {
                            let value = value.as_str().trim();
                            let value = value
                                .strip_prefix('"')
                                .and_then(|v| v.strip_suffix('"'))
                                .or_else(|| {
                                    value.strip_prefix('\'').and_then(|v| v.strip_suffix('\''))
                                })
                                .unwrap_or(value);
                            Some(value.to_owned())
                        })
                        .collect();
                    tree.insert(&key, Some(values));
                }
                _ => unreachable!(),
            }
        }
        Rule::config => {
            for child in pair.into_inner() {
                process(child, prefix, tree);
            }
        }
        _ => {}
    }
}

/// Parse the supported Linux bootconfig text syntax.
///
/// Returns an error on invalid text or unsupported assignments.
pub fn parse_text(text: &str) -> Result<BootConfig> {
    let pairs = BootconfigParser::parse(Rule::config, text).context(error::BootconfigParseSnafu)?;
    let mut tree = Node::default();
    for pair in pairs {
        process(pair, "", &mut tree);
    }
    let mut config = BootConfig::default();
    for (name, child) in tree.children {
        match name.as_str() {
            "kernel" => child.collect("", &mut config.kernel),
            "init" => child.collect("", &mut config.init),
            _ => {}
        }
    }
    ensure!(
        config
            .kernel
            .iter()
            .chain(&config.init)
            .all(|p| !p.name.is_empty()),
        error::InvalidNamespaceSnafu
    );
    Ok(config)
}
