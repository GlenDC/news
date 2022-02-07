use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};

use crate::i18n::locales::Storage;

pub fn generate_locales(dir: &str, storage: &Storage) -> Result<()> {
    fs::create_dir_all(dir)?;
    let file_path = Path::new(dir).join("locales.rs");
    let mut file = File::create(file_path)
        .with_context(|| format!("create locales rust file at {}/locales.rs", dir))?;

    let default_locales = storage
        .get_default()
        .ok_or_else(|| anyhow!("failed to get default locale in i18n storage"))?;

    let mut default_values = Vec::new();
    file.write_all(
        b"#[derive(Debug)]
pub struct Locales<'a> {
",
    )?;
    // TODO: create nested structs & support proper indenting
    for pair in default_locales.iter() {
        file.write_all(
            format!(
                "    pub {}: &'a str,\n",
                pair.path[pair.path.len() - 1].to_case(Case::Snake)
            )
            .as_bytes(),
        )?;
        default_values.push(pair);
    }
    file.write_all(
        b"}
",
    )?;

    Ok(())
}
