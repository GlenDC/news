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

/// The l18n module contains auto-generate code at build time via `build.rs.
///
/// Please see the "Cargo.toml" for the "l18n" config
/// and the "plabayo-news-builder" sibling crate for more information
/// on how it is generated. If you're IDE doesn't auto build you might
/// need to manually run `cargo build` if you do not have the
/// generated files in this mod directory yet.
///
/// - `locales.rs`: contains the Locale enum for all language variants supported by Plabayo News
///   and also the structs and constant strings (using these structs) containing all translation strings,
///   to which the formatter (if used, e.g. Markdown=md) is already applied.
/// - `pages.rs`: contains the static page content, and is used to serve the static
///   pages of Plabayo News build at compile time and served on runtime from memory
///   as a raw opaque html string.
pub mod locales;
pub mod pages;
