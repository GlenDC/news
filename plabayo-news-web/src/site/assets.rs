use actix_web_static_files;

// include generated (assets) resource files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use actix_web::dev::HttpServiceFactory;

pub fn factory() -> impl HttpServiceFactory + 'static {
    let generated = generate();
    actix_web_static_files::ResourceFiles::new("/assets", generated)
}
