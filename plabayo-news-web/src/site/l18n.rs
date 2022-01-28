use pulldown_cmark::{html, Options, Parser};
use rust_i18n::t;

pub const DEFAULT_LOCALE: &str = "en";
pub const SUPPORTED_LOCALES: &[&str] = &[DEFAULT_LOCALE, "nl", "es"];

pub fn txt(path: &str, locale: &str) -> String {
    t!(path, locale = locale)
}

pub fn md(path: &str, locale: &str) -> String {
    let input = t!(path, locale = locale);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);

    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
