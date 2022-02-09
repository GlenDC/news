use std::fs::{self, File};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use itertools::Itertools;

use crate::i18n::locales::{Storage, StringValuePathPair};

pub fn generate_locales(dir: &str, storage: &Storage) -> Result<()> {
    fs::create_dir_all(dir)?;
    let file_path = Path::new(dir).join("locales.rs");
    let file = File::create(file_path)
        .with_context(|| format!("create locales rust file at {}/locales.rs", dir))?;

    let default_locales = storage
        .get_default()
        .ok_or_else(|| anyhow!("failed to get default locale in i18n storage"))?;

    generate_locales_enum(&file, storage)?;

    let default_pairs: Vec<StringValuePathPair> = default_locales.iter().collect();

    generate_locales_strings_struct(
        &file,
        default_pairs.iter().map(|p| p.path.clone()).collect(),
    )?;

    generate_locales_strings_instance(&file, "STRINGS_DEFAULT", default_pairs.iter())?;

    for locale in storage
        .all_locales()
        .filter(|locale| locale != &storage.default_locale())
    {
        let iter = LocaleStringWithDefaultIter::new(
            storage
                .get(locale)
                .ok_or_else(|| anyhow!("failed to get strings for locale {}", locale))?
                .iter(),
            default_pairs.clone().into_iter(),
        );
        let pairs: Vec<StringValuePathPair> = iter.collect();
        generate_locales_strings_instance(
            &file,
            &format!("STRINGS_{}", locale.to_case(Case::ScreamingSnake)),
            pairs.iter(),
        )?;
    }

    Ok(())
}

fn generate_locales_enum(mut w: impl std::io::Write, storage: &Storage) -> Result<()> {
    // 1. generate enum type

    w.write_all(
        b"pub enum Locales {
",
    )?;
    for locale in storage.all_locales() {
        w.write_all(
            format!(
                "    {},
",
                locale.to_case(Case::Pascal)
            )
            .as_bytes(),
        )?;
    }
    w.write_all(
        b"}

",
    )?;

    // 2. impl conversation from str, for our enum type

    w.write_all(
        b"impl From<&str> for Locales {
    fn from(s: &str) -> Self {
        match s.to_lowercase().trim() {
",
    )?;
    for locale in storage.all_locales() {
        w.write_all(
            format!(
                r#"            "{}" => Self::{},
"#,
                locale.to_lowercase().trim(),
                locale.to_case(Case::Pascal)
            )
            .as_bytes(),
        )?;
    }
    w.write_all(
        b"            _ => DEFAULT_LOCALE,
",
    )?;
    w.write_all(
        b"        }
    }
}

",
    )?;

    // 3. generate default locale constant

    w.write_all(
        format!(
            "pub const DEFAULT_LOCALE: Locales = Locales::{};

",
            storage.default_locale().to_case(Case::Pascal)
        )
        .as_bytes(),
    )?;

    // x. all good

    Ok(())
}

fn generate_locales_strings_struct(
    mut w: impl std::io::Write,
    mut paths: Vec<Vec<String>>,
) -> Result<()> {
    let mut layer: usize = 0;
    while !paths.is_empty() {
        if layer == 0 {
            w.write_all(
                b"pub struct Strings {
",
            )?;
        }
        let mut previous: Option<String> = None;
        let mut previous_property: Option<String> = None;
        let mut retained_paths = Vec::new();
        for path in paths {
            // create new struct if needed
            let current = if layer == 0 {
                None
            } else {
                Some(path[layer - 1].clone())
            };
            if previous != current {
                w.write_all(
                    b"}

",
                )?;
                w.write_all(
                    format!(
                        "pub struct Strings{} {{
",
                        path[..layer]
                            .iter()
                            .map(|s| s.to_case(Case::Pascal))
                            .join("")
                    )
                    .as_bytes(),
                )?;
                previous = current;
            }

            let key = &path[layer];
            let current_property = Some(key.clone());
            let drop = path.len() == layer + 1;

            // write struct property
            if drop {
                // str
                w.write_all(
                    format!(
                        "    pub {}: &'static str,
",
                        key.to_lowercase().trim()
                    )
                    .as_bytes(),
                )?;
            } else if current_property != previous_property {
                // object
                w.write_all(
                    format!(
                        "    pub {}: Strings{},
",
                        key.to_lowercase().trim(),
                        path[..layer + 1]
                            .iter()
                            .map(|s| s.to_case(Case::Pascal))
                            .join("")
                    )
                    .as_bytes(),
                )?;
                previous_property = current_property;
            }

            // retain if we do not wish to drop
            if !drop {
                retained_paths.push(path);
            }
        }

        layer += 1;
        paths = retained_paths;
    }
    w.write_all(
        b"}
",
    )?;
    Ok(())
}

fn generate_locales_strings_instance<'a> (
    mut w: impl std::io::Write,
    const_name: &str,
    pairs: impl Iterator<Item = &'a StringValuePathPair>,
) -> Result<()> {
    w.write_all(
        format!(
            "
const {}: Strings = Strings{{
",
            const_name
        )
        .as_bytes(),
    )?;
    let mut previous_layer = 0;
    let mut previous_path = None;
    // for each locale string...
    for pair in pairs {
        let current_layer = pair.path.len() - 1;
        if current_layer > previous_layer {
            // handle case in case we are indenting more (creating a child)
            while current_layer > previous_layer {
                let key = &pair.path[previous_layer];
                w.write_all(
                    format!(
                        "{}{}: Strings{}{{
",
                        "    ".repeat(previous_layer + 1),
                        key,
                        pair.path[..=previous_layer]
                            .iter()
                            .map(|s| s.to_case(Case::Pascal))
                            .join(""),
                    )
                    .as_bytes(),
                )?;
                previous_layer += 1;
            }
        } else if current_layer < previous_layer {
            // as well as the case where we indenting less (ending a child)
            while current_layer < previous_layer {
                previous_layer -= 1;
                w.write_all(
                    format!(
                        "{}}},
",
                        "    ".repeat(previous_layer + 1)
                    )
                    .as_bytes(),
                )?;
            }
        } else {
            // and finally handle the cases where we go from one nested child to another
            let mut overlap_layer = 0;
            if let Some(previous_path) = previous_path {
                for (key_a, key_b) in pair.path.iter().zip(previous_path) {
                    if key_a != key_b {
                        break;
                    }
                    overlap_layer += 1;
                }
                if overlap_layer < previous_layer {
                    for idx in 0..(previous_layer - overlap_layer) {
                        w.write_all(
                            format!(
                                "{}}},
",
                                "    ".repeat(previous_layer - idx)
                            )
                            .as_bytes(),
                        )?;
                    }
                    while overlap_layer < previous_layer {
                        let key = &pair.path[overlap_layer];
                        w.write_all(
                            format!(
                                "{}{}: Strings{}{{
",
                                "    ".repeat(overlap_layer + 1),
                                key,
                                pair.path[..=overlap_layer]
                                    .iter()
                                    .map(|s| s.to_case(Case::Pascal))
                                    .join(""),
                            )
                            .as_bytes(),
                        )?;
                        overlap_layer += 1;
                    }
                }
            }
        }
        // write the actual locale string...
        let key = &pair.path[previous_layer];
        w.write_all(
            format!(
                r#################"{}{}: r################"{}"################,
"#################,
                "    ".repeat(previous_layer + 1),
                key,
                pair.value
            )
            .as_bytes(),
        )?;
        // keep track of the previous path to be handle the more complex nesting cases
        previous_path = Some(&pair.path);
    }
    // add all the final curly brackets... including the last one
    while previous_layer > 0 {
        previous_layer -= 1;
        w.write_all(
            format!(
                "{}}},
",
                "    ".repeat(previous_layer + 1)
            )
            .as_bytes(),
        )?;
    }
    w.write_all(
        b"};
",
    )?;
    Ok(())
}

struct LocaleStringWithDefaultIter<
    T: Iterator<Item = StringValuePathPair>,
    U: Iterator<Item = StringValuePathPair>,
> {
    pairs: Box<T>,
    default_pairs: Box<U>,
    next_default_pair: Option<StringValuePathPair>,
}

impl<
        T: Iterator<Item = StringValuePathPair>,
        U: Iterator<Item = StringValuePathPair>,
    > LocaleStringWithDefaultIter<T, U>
{
    pub fn new(pairs: T, mut default_pairs: U) -> LocaleStringWithDefaultIter<T, U> {
        let next_default_pair = default_pairs.next();
        LocaleStringWithDefaultIter {
            pairs: Box::new(pairs),
            default_pairs: Box::new(default_pairs),
            next_default_pair: next_default_pair,
        }
    }
}

impl<
        T: Iterator<Item = StringValuePathPair>,
        U: Iterator<Item = StringValuePathPair>,
    > Iterator for LocaleStringWithDefaultIter<T, U>
{
    type Item = StringValuePathPair;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(&mut self.next_default_pair, None) {
            None => None,
            Some(next_default_pair) => {
                while let Some(pair) = self.pairs.next() {
                    if pair == next_default_pair {
                        self.next_default_pair = self.default_pairs.next();
                        return Some(pair.clone());
                    }
                    if pair > next_default_pair  {
                        continue;
                    }
                    // missing keys, we'll fill up...
                    self.next_default_pair = self.default_pairs.next();
                    return Some(StringValuePathPair {
                        path: next_default_pair.path.clone(),
                        value: format!(
                            "&DEFAULT_LOCALE.{}",
                            next_default_pair
                                .path
                                .iter()
                                .map(|s| s.to_case(Case::Pascal))
                                .join("")
                        ),
                    });
                }
                self.next_default_pair = self.default_pairs.next();
                None
            }
        }
    }
}
