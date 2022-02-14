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

use anyhow::Result;

mod codegen;
mod config;
mod locales;

/// build the i18n locale structs and (Askama) templates
/// for the project
pub fn build(cargo_toml: &str) -> Result<()> {
    let i18n_cfg = config::load(cargo_toml)?;
    let locales_storage = locales::Storage::load(&i18n_cfg.path, &i18n_cfg.locales[..])?;
    codegen::generate_all(&i18n_cfg.out, &locales_storage, &i18n_cfg.pages)
}
