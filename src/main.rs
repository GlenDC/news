use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use actix_web::http::StatusCode;
use actix_web_static_files;
use askama::Template;
use fnv::FnvHasher;
use std::hash::Hasher;
use rust_i18n::{t, i18n};

// init yaml-based translations
i18n!("locales");

// include generated (assets) resource files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

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
    pub fn new(locale: &str, param_locale: &str) -> SiteLocales {
        SiteLocales{
            name: t!("site.name", locale=locale),
            locale: String::from(param_locale),
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
#[template(path = "pages/item.html")]
struct PageItem<'a> {
    site_info: SiteInfo,
    q: &'a str,
    i18n: PageItemLocales,
}

struct PageItemLocales{
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

async fn page_unknown(locale: &str, param_locale: &str, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNotFound {
        site_info: data.info,
        i18n: PageNotFoundLocales{
            site: SiteLocales::new(locale, param_locale),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_home(data: web::Data<SiteState>) -> Result<HttpResponse> {
    // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
    page_news(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await
}

async fn page_home_with_locale_or_path(data: web::Data<SiteState>, path: web::Path<(String,)>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    match path.into_inner().0.to_lowercase().as_str() {
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "news" => page_news(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await,
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "search" => page_search(DEFAULT_LOCALE, DEFAULT_LOCALE, data, query).await,
        "all" => {
            // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
            // and set special flag somehow when fetching items to select all content
            page_news(DEFAULT_LOCALE, "all", data).await
        },
        "assets" => Ok(HttpResponse::new(StatusCode::NOT_FOUND)),
        locale => if ALL_LOCALES.iter().any(|v| v == &locale) {
            page_news(locale, locale, data).await
        } else {
            page_unknown(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await
        },
    }
}

async fn page_home_with_locale_and_path(data: web::Data<SiteState>, path: web::Path<(String, String, String)>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let path = path.into_inner();
    let mut locale = path.0.to_lowercase();

    if locale == "assets" {
        // handle the case where a source is not found
        return Ok(HttpResponse::new(StatusCode::NOT_FOUND));
    }

    let param_locale = locale.clone();
    if locale == "all" {
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        // and set special flag somehow when fetching items to select all content
        locale = String::from(DEFAULT_LOCALE);
    } else if !ALL_LOCALES.iter().any(|v| v == &locale) {
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        // TODO: add suggestion related to locale?!
        return page_unknown(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await;
    }
    if !path.2.is_empty() {
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        // TODO: add suggestion related to path?!
        return page_unknown(&locale, &param_locale, data).await;
    }
    match path.1.to_lowercase().as_str() {
        "news" => page_news(&locale, &param_locale, data).await,
        "search" => page_search(&locale, &param_locale, data, query).await,
        "item" => page_item(&locale, &param_locale, data, query).await,
        DEFAULT_LOCALE => page_news(&locale, &param_locale, data).await,
        _ => page_unknown(&locale, &param_locale, data).await,
    }
}

async fn page_news(locale: &str, param_locale: &str, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = PageNews {
        site_info: data.info,
        i18n: PageNewsLocales{
            site: SiteLocales::new(locale, param_locale),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_item(locale: &str, param_locale: &str, data: web::Data<SiteState>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageItem {
        site_info: data.info,
        q: q,
        i18n: PageItemLocales{
            site: SiteLocales::new(locale, param_locale),
        },
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(locale: &str, param_locale: &str, data: web::Data<SiteState>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageSearch {
        site_info: data.info,
        q: q,
        i18n: PageSearchLocales{
            site: SiteLocales::new(locale, param_locale),
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
        let generated = generate();
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
                info: site_info,
            })
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(middleware::normalize::TrailingSlash::Trim))
            .service(actix_web_static_files::ResourceFiles::new("/assets", generated))
            .service(web::resource("/").route(web::get().to(page_home)))
            .service(web::resource("/{locale_or_page}").route(web::get().to(page_home_with_locale_or_path)))
            .service(web::resource("/{locale}/{page}{tail:.*}").route(web::get().to(page_home_with_locale_and_path)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
