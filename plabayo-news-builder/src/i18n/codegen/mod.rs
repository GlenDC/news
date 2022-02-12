use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::i18n::locales::Storage;

mod locales;

pub fn generate_all(dir: &str, storage: &Storage) -> Result<()> {
    fs::create_dir_all(dir)?;

    locales::generate_locales(&Path::new(dir).join("locales.rs"), storage)
}
