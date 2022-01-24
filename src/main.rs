use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use askama::Template;
use fnv::FnvHasher;
use plabayo_news::l18n;
use std::hash::Hasher;

// include generated (assets) resource files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const DEFAULT_LOCALE: &'static str = "en";
const ALL_LOCALES: &'static [&'static str] = &[DEFAULT_LOCALE, "nl", "es"];

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
        SiteLocales {
            name: l18n::txt("site.name", locale),
            locale: String::from(param_locale),
            nav: NavLocales {
                header: NavHeaderLocales {
                    news: l18n::txt("site.nav.header.news", locale),
                    past: l18n::txt("site.nav.header.past", locale),
                    comments: l18n::txt("site.nav.header.comments", locale),
                    ask: l18n::txt("site.nav.header.ask", locale),
                    show: l18n::txt("site.nav.header.show", locale),
                    events: l18n::txt("site.nav.header.events", locale),
                    submit: l18n::txt("site.nav.header.submit", locale),
                    login: l18n::txt("site.nav.header.login", locale),
                },
                footer: NavFooterLocales {
                    guidelines: l18n::txt("site.nav.footer.guidelines", locale),
                    faq: l18n::txt("site.nav.footer.faq", locale),
                    api: l18n::txt("site.nav.footer.api", locale),
                    security: l18n::txt("site.nav.footer.security", locale),
                    legal: l18n::txt("site.nav.footer.legal", locale),
                    contact: l18n::txt("site.nav.footer.contact", locale),
                    search: l18n::txt("site.nav.footer.search", locale),
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

struct PageNotFoundLocales {
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct PageNews {
    site_info: SiteInfo,
    i18n: PageNewsLocales,
}

struct PageNewsLocales {
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/item.html")]
struct PageItem<'a> {
    site_info: SiteInfo,
    q: &'a str,
    i18n: PageItemLocales,
}

struct PageItemLocales {
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/search.html")]
struct PageSearch<'a> {
    site_info: SiteInfo,
    q: &'a str,
    i18n: PageSearchLocales,
}

struct PageSearchLocales {
    site: SiteLocales,
}

#[derive(Template)]
#[template(path = "pages/security.html", escape = "none")]
struct PageSecurity {
    site_info: SiteInfo,
    i18n: PageSecurityLocales,
}

struct PageSecurityLocales {
    site: SiteLocales,
    intro: String,
    reports: String,
}

async fn page_unknown(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = PageNotFound {
        site_info: data.info,
        i18n: PageNotFoundLocales {
            site: SiteLocales::new(locale, param_locale),
        },
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_home(data: web::Data<SiteState>) -> Result<HttpResponse> {
    // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
    page_news(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await
}

async fn page_home_with_locale_or_path(
    data: web::Data<SiteState>,
    path: web::Path<(String,)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    match path.into_inner().0.to_lowercase().as_str() {
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "news" => page_news(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await,
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "search" => page_search(DEFAULT_LOCALE, DEFAULT_LOCALE, data, query).await,
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "security" => page_security(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await,
        // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
        "item" => page_item(DEFAULT_LOCALE, DEFAULT_LOCALE, data, query).await,
        "all" => {
            // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
            // and set special flag somehow when fetching items to select all content
            page_news(DEFAULT_LOCALE, "all", data).await
        }
        "assets" => Ok(HttpResponse::new(StatusCode::NOT_FOUND)),
        locale => {
            if ALL_LOCALES.iter().any(|v| v == &locale) {
                page_news(locale, locale, data).await
            } else {
                page_unknown(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await
            }
        }
    }
}

async fn page_home_with_locale_and_path(
    data: web::Data<SiteState>,
    path: web::Path<(String, String, String)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
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
        "security" => page_security(&locale, &param_locale, data).await,
        "item" => page_item(&locale, &param_locale, data, query).await,
        DEFAULT_LOCALE => page_news(&locale, &param_locale, data).await,
        _ => page_unknown(&locale, &param_locale, data).await,
    }
}

async fn page_news(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = PageNews {
        site_info: data.info,
        i18n: PageNewsLocales {
            site: SiteLocales::new(locale, param_locale),
        },
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageSearch {
        site_info: data.info,
        q: q,
        i18n: PageSearchLocales {
            site: SiteLocales::new(locale, param_locale),
        },
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_security(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = PageSecurity {
        site_info: data.info,
        i18n: PageSecurityLocales {
            site: SiteLocales::new(locale, param_locale),
            intro: l18n::md("page.security.intro", locale),
            reports: l18n::md("page.security.reports", locale),
        },
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_item(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let q = match query.get("q") {
        Some(s) => &s,
        None => "",
    };
    let s = PageItem {
        site_info: data.info,
        q: q,
        i18n: PageItemLocales {
            site: SiteLocales::new(locale, param_locale),
        },
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || {
        let generated = generate();
        let site_info = SiteInfo {
            version: (|| -> u64 {
                let timestamp_str = env!("VERGEN_BUILD_TIMESTAMP");
                let mut hasher: FnvHasher = Default::default();
                hasher.write(timestamp_str.as_bytes());
                hasher.finish()
            })(),
        };
        App::new()
            .data(SiteState { info: site_info })
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .wrap(middleware::Compress::default())
            .service(actix_web_static_files::ResourceFiles::new(
                "/assets", generated,
            ))
            .service(web::resource("/").route(web::get().to(page_home)))
            .service(
                web::resource("/{locale_or_page}")
                    .route(web::get().to(page_home_with_locale_or_path)),
            )
            .service(
                web::resource("/{locale}/{page}{tail:.*}")
                    .route(web::get().to(page_home_with_locale_and_path)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
