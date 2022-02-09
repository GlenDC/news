use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Error, Result};
use itertools::Itertools;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use serde_yaml::{from_reader, from_value, Value};

#[derive(Debug)]
pub struct Storage {
    default_locale: String,
    locale_to_values_map: HashMap<String, Locales>,
}

impl Storage {
    pub fn load<T: AsRef<str>>(path: &str, supported_locales: &[T]) -> Result<Storage> {
        let mut locale_to_values_map = HashMap::new();
        for supported_locale in supported_locales.iter().map(|r| r.as_ref()) {
            let locales = Locales::load(path, supported_locale)?;
            locale_to_values_map.insert(supported_locale.to_owned(), locales);
        }
        Ok(Storage {
            default_locale: supported_locales[0].as_ref().to_owned(),
            locale_to_values_map,
        })
    }

    pub fn default_locale(&self) -> &str {
        self.default_locale.as_str()
    }

    pub fn all_locales(&self) -> impl Iterator<Item = &str> {
        self.locale_to_values_map
            .keys()
            .map(|k| k.as_str())
            .sorted()
    }

    pub fn get_default(&self) -> Option<&Locales> {
        self.get(&self.default_locale)
    }

    pub fn get(&self, locale: &str) -> Option<&Locales> {
        self.locale_to_values_map.get(locale)
    }
}

#[derive(Debug)]
pub struct Locales {
    values: HashMap<String, Value>,
}

impl Locales {
    pub fn load(path: &str, locale: &str) -> Result<Locales> {
        let locale_path = Path::new(path).join(format!("{}.yml", locale));
        let locales_file = File::open(locale_path)
            .with_context(|| format!("open locale file {}/{}.yml", path, locale))?;
        let values: HashMap<String, Value> = from_reader(locales_file)
            .with_context(|| format!("load locale file {}/{}.yml", path, locale))?;
        Ok(Locales { values })
    }

    pub fn iter(&self) -> impl Iterator<Item = StringValuePathPair> + '_ {
        ValueIter::new(&self.values).sorted()
    }
}

pub struct ValueIter<'a> {
    stack: Vec<ValuePathPairRef<'a>>,
}

struct ValuePathPairRef<'a> {
    value: &'a Value,
    path: Vec<String>,
}

#[derive(Eq, Clone)]
pub struct StringValuePathPair {
    pub value: String,
    pub path: Vec<String>,
}

impl PartialEq for StringValuePathPair {
    fn eq(&self, other: &StringValuePathPair) -> bool {
        if self.path.len() != other.path.len() {
            return false;
        }
        for i in 0..self.path.len() {
            if self.path[i] != other.path[i] {
                return false;
            }
        }
        true
    }
}

impl Ord for StringValuePathPair {
    fn cmp(&self, other: &StringValuePathPair) -> Ordering {
        for i in 0..self.path.len() {
            let c = self.path[i].cmp(&other.path[i]);
            if i >= other.path.len() {
                return Ordering::Less;
            }
            if c != Ordering::Equal {
                return c;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for StringValuePathPair {
    fn partial_cmp(&self, other: &StringValuePathPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ValueIter<'a> {
    pub fn new(values: &'a HashMap<String, Value>) -> ValueIter<'a> {
        let mut stack = Vec::with_capacity(values.len());
        for (k, v) in values {
            stack.push(ValuePathPairRef {
                value: v,
                path: vec![k.clone()],
            });
        }
        ValueIter { stack }
    }
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = StringValuePathPair;

    fn next(&mut self) -> Option<StringValuePathPair> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(pair_ref) => match pair_ref.value {
                    Value::Null => continue,
                    Value::Bool(b) => {
                        return Some(StringValuePathPair {
                            value: (if *b { "true" } else { "false" }).to_owned(),
                            path: pair_ref.path,
                        })
                    }
                    Value::Number(n) => {
                        return Some(StringValuePathPair {
                            value: format!("{}", n),
                            path: pair_ref.path,
                        })
                    }
                    Value::String(s) => {
                        return Some(StringValuePathPair {
                            value: s.clone(),
                            path: pair_ref.path,
                        })
                    }
                    Value::Sequence(_) => continue,
                    Value::Mapping(m) => match TypedValue::try_from(pair_ref.value) {
                        Ok(tv) => {
                            return Some(StringValuePathPair {
                                value: tv.to_string(),
                                path: pair_ref.path,
                            })
                        }
                        Err(_) => {
                            for (k, v) in m {
                                if let Some(key) = k.as_str() {
                                    let mut path = pair_ref.path.clone();
                                    path.push(key.to_owned());
                                    self.stack.push(ValuePathPairRef { value: v, path });
                                }
                            }
                        }
                    },
                },
            }
        }
    }
}

#[derive(Debug, Deserialize)]
enum ValueFormat {
    #[serde(rename = "txt")]
    Text,
    #[serde(rename = "md")]
    Markdown,
}

#[derive(Debug, Deserialize)]
struct TypedValue {
    value: String,
    format: Option<ValueFormat>,
}

impl ToString for TypedValue {
    fn to_string(&self) -> String {
        let format = self.format.as_ref().unwrap_or(&ValueFormat::Text);
        match format {
            ValueFormat::Text => self.value.clone(),
            ValueFormat::Markdown => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(&self.value, options);

                let mut output = String::new();
                html::push_html(&mut output, parser);

                output
            }
        }
    }
}

impl TryFrom<&Value> for TypedValue {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.as_str() {
            return Ok(TypedValue {
                value: s.to_owned(),
                format: Some(ValueFormat::Text),
            });
        }
        let value: TypedValue = from_value(value.clone())?;
        Ok(value)
    }
}
