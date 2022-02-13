use actix_web_static_files::resource_dir;
use anyhow::Result;
use vergen::{vergen, Config};

use plabayo_news_builder::i18n;

fn main() -> Result<()> {
    // Generate the default 'cargo:' instruction output
    vergen(Config::default())?;

    // build the i18n locale structs and (Askama) templates
    // for the website's static pages.
    i18n::build("./Cargo.toml")?;

    // Bundle static resources so we can serve these from memory,
    // and make the setup of the news web server easier.
    resource_dir("./site/assets").build()?;

    // All good.
    Ok(())
}
