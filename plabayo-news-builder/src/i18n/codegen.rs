use std::fs::{self, File};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use itertools::Itertools;

use crate::i18n::locales::Storage;

pub fn generate_locales(dir: &str, storage: &Storage) -> Result<()> {
    fs::create_dir_all(dir)?;
    let file_path = Path::new(dir).join("locales.rs");
    let file = File::create(file_path)
        .with_context(|| format!("create locales rust file at {}/locales.rs", dir))?;

    let default_locales = storage
        .get_default()
        .ok_or_else(|| anyhow!("failed to get default locale in i18n storage"))?;

    generate_locales_enum(&file, storage)?;

    generate_locales_strings_struct(
        &file,
        default_locales.iter().map(|p| p.path.clone()).collect(),
    )?;

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
        paths.retain(|path| {
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
                )
                .unwrap();
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
                )
                .unwrap();
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
                )
                .unwrap();
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
                )
                .unwrap();
                previous_property = current_property;
            }

            // retain if we do not wish to drop
            !drop
        });

        layer += 1;
    }
    w.write_all(
        b"}
",
    )?;
    Ok(())
}
