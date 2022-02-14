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

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::i18n::config::StaticPages;
use crate::i18n::locales::Storage;

mod common;
mod locales;
mod pages;

pub fn generate_all(dir: &str, storage: &Storage, pages: &StaticPages) -> Result<()> {
    fs::create_dir_all(dir)?;

    locales::generate_locales(&Path::new(dir).join("locales.rs"), storage)?;
    pages::generate_pages(&Path::new(dir).join("pages.rs"), storage, pages)
}
