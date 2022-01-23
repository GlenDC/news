use std::collections::HashMap;

use actix_files as fs;
use actix_web::{middleware, web, guard, App, HttpResponse, HttpServer, Result};
use askama::Template;
use fnv::FnvHasher;
use std::hash::Hasher;
use rust_i18n::{t, i18n};

// init yaml-based translations
i18n!("locales");

struct SiteState {
    info: SiteInfo,
}

struct SiteLocales {
    name: String,
    nav: NavLocales,
}

impl SiteLocales {
    pub fn new() -> SiteLocales {
        SiteLocales{
            name: t!("site.name"),
            nav: NavLocales{
                header: NavHeaderLocales{
                    news: t!("site.nav.header.news"),
                    past: t!("site.nav.header.past"),
                    comments: t!("site.nav.header.comments"),
                    ask: t!("site.nav.header.ask"),
                    show: t!("site.nav.header.show"),
                    events: t!("site.nav.header.events"),
                    submit: t!("site.nav.header.submit"),
                    login: t!("site.nav.header.login"),
                },
                footer: NavFooterLocales{
                    guidelines: t!("site.nav.footer.guidelines"),
                    faq: t!("site.nav.footer.faq"),
                    lists: t!("site.nav.footer.lists"),
                    api: t!("site.nav.footer.api"),
                    security: t!("site.nav.footer.security"),
                    legal: t!("site.nav.footer.legal"),
                    contact: t!("site.nav.footer.contact"),
                    search: t!("site.nav.footer.search"),
                },
            },
        }
    }
}

struct NavLocales {
    header: NavHeaderLocales,
    footer: NavFooterLocales,
}

struct NavHeaderLocales {
    news: String,
    past: String,
    comments: String,
    ask: String,
    show: String,
    events: String,
    submit: String,
    login: String,
}

struct NavFooterLocales {
    guidelines: String,
    faq: String,
    lists: String,
    api: String,
    security: String,
    legal: String,
    contact: String,
    search: String,
}

#[derive(Clone, Copy, Debug)]
struct SiteInfo {
    version: u64,
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
struct PageNotFound {
    site_info: SiteInfo,
    i18n: PageNotFoundLocales,
}

struct PageNotFoundLocales{
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct PageNews {
    site_info: SiteInfo,
    i18n: PageNewsLocales,
}

struct PageNewsLocales{
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/search.html")]
struct PageSearch<'a> {
    site_info: SiteInfo,
    q: &'a str,
    i18n: PageSearchLocales,
}

struct PageSearchLocales{
    site: SiteLocales,
}

async fn page_news(data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNews {
        site_info: data.info,
        i18n: PageNewsLocales{
            site: SiteLocales::new(),
        },
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
        i18n: PageSearchLocales{
            site: SiteLocales::new(),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || {
        let site_info = SiteInfo{
            version: (|| -> u64 {
                let timestamp_str = env!("VERGEN_BUILD_TIMESTAMP");
                let mut hasher: FnvHasher = Default::default();
                hasher.write(timestamp_str.as_bytes());
                hasher.finish()
            })(),
        };
        App::new()
            .data(SiteState {
                info: site_info.clone(),
            })
            .wrap(middleware::Logger::default())
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(move || {
                        let s = PageNotFound {
                            site_info: site_info,
                            i18n: PageNotFoundLocales{
                                site: SiteLocales::new(),
                            },
                        }.render().unwrap();
                        HttpResponse::Ok().content_type("text/html").body(s)
                    }),
            )
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(web::resource("/").route(web::get().to(page_news)))
            .service(web::resource("/news").route(web::get().to(page_news)))
            .service(web::resource("/search").route(web::get().to(page_search)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
