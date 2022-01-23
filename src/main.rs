use std::collections::HashMap;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use fnv::FnvHasher;
use std::hash::Hasher;

struct SiteState {
    info: SiteInfo,
}

#[derive(Clone, Copy, Debug)]
struct SiteInfo {
    version: u64,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct PageNews {
    site_info: SiteInfo,
}

#[derive(Template)]
#[template(path = "pages/search.html")]
struct PageSearch<'a> {
    site_info: SiteInfo,
    q: &'a str,
}

async fn page_news(data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNews {
        site_info: data.info,
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(data: web::Data<SiteState>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageSearch {
        site_info: data.info,
        q: q,
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(SiteState {
                info: SiteInfo{
                    version: (|| -> u64 {
                        let timestamp_str = env!("VERGEN_BUILD_TIMESTAMP");
                        let mut hasher: FnvHasher = Default::default();
                        hasher.write(timestamp_str.as_bytes());
                        hasher.finish()
                    })(),
                },
            })
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(web::resource("/").route(web::get().to(page_news)))
            .service(web::resource("/news").route(web::get().to(page_news)))
            .service(web::resource("/search").route(web::get().to(page_search)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
