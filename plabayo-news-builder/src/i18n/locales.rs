use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Error, Result};
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
        self.locale_to_values_map.keys().map(|k| k.as_str())
    }
}

#[derive(Debug)]
struct Locales {
    values: HashMap<String, Value>,
}

impl Locales {
    pub fn load(path: &str, locale: &str) -> Result<Locales> {
        let locale_path = Path::new(path).join(format!("{}.yml", locale));
        let locales_file = File::open(locale_path)
            .with_context(|| format!("open locale file {}/{}.yml", path, locale,))?;
        let values: HashMap<String, Value> = from_reader(locales_file)
            .with_context(|| format!("load locale file {}/{}.yml", path, locale,))?;
        Ok(Locales { values })
    }

    pub fn get<'a>(&self, path: impl Iterator<Item = &'a str>) -> Option<TypedValue> {
        None
    }
}

#[derive(Deserialize)]
enum ValueFormat {
    #[serde(rename = "txt")]
    Text,
    #[serde(rename = "md")]
    Markdown,
}

#[derive(Deserialize)]
struct TypedValue {
    value: String,
    format: ValueFormat,
}

impl ToString for TypedValue {
    fn to_string(&self) -> String {
        match self.format {
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
                format: ValueFormat::Text,
            });
        }
        let value: TypedValue = from_value(value.clone())?;
        Ok(value)
    }
}
