use actix_web_static_files::resource_dir;
use anyhow::Result;
use vergen::{vergen, Config};

fn main() -> Result<()> {
    // Generate the default 'cargo:' instruction output
    vergen(Config::default())?;
    // Bundle static resources so we can serve these from memory,
    // and make the setup of the news web server easier.
    resource_dir("./site/assets").build()?;
    // All good.
    Ok(())
}
