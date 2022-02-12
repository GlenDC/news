use std::fs::File;
use std::path::Path;

use anyhow::Result;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub fn generate_pages(file_path: &Path, storage: &Storage) -> Result<()> {
    let file = File::create(file_path)
        .with_context(|| format!("create locales rust file at {}", file_path.display()))?;

    Ok(())
}