use anyhow::Result;

mod config;
mod locales;

/// build the i18n locale structs and (Askama) templates
/// for the project
pub fn build(cargo_toml: &str) -> Result<()> {
    let i18n_cfg = config::load(cargo_toml)?;
    let locales_storage = locales::Storage::load(&i18n_cfg.path, &i18n_cfg.locales[..])?;
    println!("{:?}", locales_storage);
    Ok(())
}
