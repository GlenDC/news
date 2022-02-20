use anyhow::Result;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct CargoConfig {
    package: PackageConfig,
}

#[derive(Deserialize)]
struct PackageConfig {
    metadata: MetaDataConfig,
}

// Plabayo News
// Copyright (C) 2021  Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[derive(Deserialize)]
struct MetaDataConfig {
    i18n: I18n,
}

#[derive(Debug, Deserialize)]
pub struct I18n {
    pub locales: Vec<String>,
    pub path: String,
    pub out: String,
    pub pages: Pages,
}

#[derive(Debug, Deserialize)]
pub struct Pages {
    pub path: String,
    pub not_found: String,
    pub templates_dir: String,
    #[serde(rename = "static")]
    pub static_pages: Vec<String>,
}

/// Load the i18n config from the package's Cargo.toml metadata.
pub fn load(cargo_toml: &str) -> Result<I18n> {
    let content = std::fs::read_to_string(cargo_toml)?;
    let cfg: CargoConfig = toml::from_str(&content)?;
    Ok(cfg.package.metadata.i18n)
}
