use std::collections::BTreeMap;

use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Result};
use askama::Template;

use crate::site::l18n::locales::Locale;
use crate::site::state::SiteState;
use crate::site::templates::pages;

pub fn factory() -> impl HttpServiceFactory + 'static {
    web::resource("/{resource:.*}").route(web::get().to(serve_page))
}

async fn serve_page(
    data: web::Data<SiteState>,
    path: web::Path<(String,)>,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let locale = match query.get("loc") {
        None => Locale::default(),
        Some(s) => Locale::from(s.as_str()),
    };
    match path.into_inner().0.to_lowercase().as_str() {
        "" | "home" | "index" => page_news(locale, "/", data).await,
        "new" | "news" => page_news(locale, "/news", data).await,
        "search" => page_search(locale, "/search", data, query).await,
        "security" => page_security(locale, "/security", data).await,
        "item" => page_item(locale, "/item", data, query).await,
        _ => page_unknown(locale, data).await,
    }
}

async fn page_unknown(locale: Locale, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = pages::NotFound::new(locale, "/", &data.info)
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_news(locale: Locale, path: &str, data: web::Data<SiteState>) -> Result<HttpResponse> {
    let s = pages::News::new(locale, path, &data.info).render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_item(
    locale: Locale,
    path: &str,
    data: web::Data<SiteState>,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Item::new(locale, path, &data.info, &query.into_inner())
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(
    locale: Locale,
    path: &str,
    data: web::Data<SiteState>,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Search::new(locale, path, &data.info, &query.into_inner())
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_security(
    locale: Locale,
    path: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = pages::Security::new(locale, path, &data.info)
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
