use serde::Deserialize;
use anyhow::Result;
use toml;

#[derive(Deserialize)]
struct CargoConfig {
    package: PackageConfig,
}

#[derive(Deserialize)]
struct PackageConfig {
    metadata: MetaDataConfig,
}

#[derive(Deserialize)]
struct MetaDataConfig {
    i18n: I18n,
}

#[derive(Debug, Deserialize)]
pub struct I18n {
    pub locales: Vec<String>,
    pub path: String,
}

/// Load the i18n config from the package's Cargo.toml metadata.
pub fn load(cargo_toml: &str) -> Result<I18n> {
    let content = std::fs::read_to_string(cargo_toml)?;
    let cfg: CargoConfig = toml::from_str(&content)?;
    Ok(cfg.package.metadata.i18n)
}
