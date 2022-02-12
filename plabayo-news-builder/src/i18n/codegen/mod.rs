use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::i18n::config::StaticPages;
use crate::i18n::locales::Storage;

mod locales;
mod pages;

pub fn generate_all(dir: &str, storage: &Storage, pages: &StaticPages) -> Result<()> {
    fs::create_dir_all(dir)?;

    locales::generate_locales(&Path::new(dir).join("locales.rs"), storage)?;
    pages::generate_pages(&Path::new(dir).join("pages.rs"), storage, pages)
}
