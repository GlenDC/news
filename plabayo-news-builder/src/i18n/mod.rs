use anyhow::Result;

mod config;
mod locales;
mod codegen;

/// build the i18n locale structs and (Askama) templates
/// for the project
pub fn build(cargo_toml: &str) -> Result<()> {
    let i18n_cfg = config::load(cargo_toml)?;
    let locales_storage = locales::Storage::load(&i18n_cfg.path, &i18n_cfg.locales[..])?;
    codegen::generate_locales(&i18n_cfg.out, &locales_storage)
}
