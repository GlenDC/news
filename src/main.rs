use std::collections::HashMap;

use actix_files as fs;
use actix_web::{middleware, web, guard, App, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use askama::Template;
use fnv::FnvHasher;
use std::hash::Hasher;
use rust_i18n::{t, i18n};

// init yaml-based translations
i18n!("locales");

const DEFAULT_LOCALE: &'static str = "en";
const ALL_LOCALES: &'static [&'static str] = &[
    DEFAULT_LOCALE,
    "nl",
    "es",
];

struct SiteState {
    info: SiteInfo,
}

struct SiteLocales {
    name: String,
    locale: String,
    nav: NavLocales,
}

impl SiteLocales {
    pub fn new(locale: &str) -> SiteLocales {
        SiteLocales{
            name: t!("site.name", locale=locale),
            locale: String::from(locale),
            nav: NavLocales{
                header: NavHeaderLocales{
                    news: t!("site.nav.header.news", locale=locale),
                    past: t!("site.nav.header.past", locale=locale),
                    comments: t!("site.nav.header.comments", locale=locale),
                    ask: t!("site.nav.header.ask", locale=locale),
                    show: t!("site.nav.header.show", locale=locale),
                    events: t!("site.nav.header.events", locale=locale),
                    submit: t!("site.nav.header.submit", locale=locale),
                    login: t!("site.nav.header.login", locale=locale),
                },
                footer: NavFooterLocales{
                    guidelines: t!("site.nav.footer.guidelines", locale=locale),
                    faq: t!("site.nav.footer.faq", locale=locale),
                    lists: t!("site.nav.footer.lists", locale=locale),
                    api: t!("site.nav.footer.api", locale=locale),
                    security: t!("site.nav.footer.security", locale=locale),
                    legal: t!("site.nav.footer.legal", locale=locale),
                    contact: t!("site.nav.footer.contact", locale=locale),
                    search: t!("site.nav.footer.search", locale=locale),
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

async fn page_unknown(locale: &str, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNotFound {
        site_info: data.info,
        i18n: PageNotFoundLocales{
            site: SiteLocales::new(locale),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_home(data: web::Data<SiteState>) -> Result<HttpResponse> {
    page_news(DEFAULT_LOCALE, data).await
}

async fn page_home_with_locale_or_path(data: web::Data<SiteState>, path: web::Path<(String,)>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    match path.into_inner().0.to_lowercase().as_str() {
        "news" => page_news(DEFAULT_LOCALE, data).await,
        "search" => page_search(DEFAULT_LOCALE, data, query).await,
        locale => if ALL_LOCALES.iter().any(|v| v == &locale) {
            page_news(locale, data).await
        } else {
            page_unknown(DEFAULT_LOCALE, data).await
        },
    }
}

async fn page_home_with_locale_and_path(data: web::Data<SiteState>, path: web::Path<(String, String)>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let path = path.into_inner();
    let locale = path.0.to_lowercase();
    if !ALL_LOCALES.iter().any(|v| v == &locale) {
        // TODO: add suggestion related to locale?!
        return page_unknown(DEFAULT_LOCALE, data).await;
    }
    match path.1.to_lowercase().as_str() {
        "news" => page_news(&locale, data).await,
        "search" => page_search(&locale, data, query).await,
        DEFAULT_LOCALE => page_news(&locale, data).await,
        _ => page_unknown(&locale, data).await,
    }
}

async fn page_news(locale: &str, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNews {
        site_info: data.info,
        i18n: PageNewsLocales{
            site: SiteLocales::new(locale),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(locale: &str, data: web::Data<SiteState>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageSearch {
        site_info: data.info,
        q: q,
        i18n: PageSearchLocales{
            site: SiteLocales::new(locale),
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
                    .to(move || page_unknown(DEFAULT_LOCALE, Data::new(SiteState {
                        info: site_info.clone(),
                    }))),
            )
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(web::resource("/").route(web::get().to(page_home)))
            .service(web::resource("/{locale_or_page}").route(web::get().to(page_home_with_locale_or_path)))
            .service(web::resource("/{locale}/{page}").route(web::get().to(page_home_with_locale_and_path)))
            /*
            .service(web::resource("/news").route(web::get().to(page_news)))
            .service(web::resource("/search").route(web::get().to(page_search)))
            */
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
