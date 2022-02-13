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
