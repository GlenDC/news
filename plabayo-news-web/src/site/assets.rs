use actix_web_static_files;

// include generated (assets) resource files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use actix_web::dev::HttpServiceFactory;

pub const ROOT: &str = "assets";

pub fn factory() -> impl HttpServiceFactory + 'static {
    let generated = generate();
    actix_web_static_files::ResourceFiles::new(format!("/{}", ROOT).as_str(), generated)
}
