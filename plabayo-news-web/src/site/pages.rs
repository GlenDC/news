use std::collections::BTreeMap;

use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, error::ErrorInternalServerError, Result};
use askama::Template;

use crate::site::l18n::locales::Locale;
use crate::site::l18n::pages::static_response;
use crate::site::templates::pages;

pub fn factory() -> impl HttpServiceFactory + 'static {
    web::resource("/{resource:.*}").route(web::get().to(serve_page))
}

async fn serve_page(
    path: web::Path<(String,)>,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let locale = match query.get("loc") {
        None => Locale::default(),
        Some(s) => Locale::from(s.as_str()),
    };
    match path.into_inner().0.to_lowercase().as_str() {
        "" | "home" | "index" => page_news(locale, "/").await,
        "new" | "news" => page_news(locale, "/news").await,
        "search" => page_search(locale, "/search", query).await,
        "item" => page_item(locale, "/item", query).await,
        endpoint => Ok(static_response(locale, endpoint).await),
    }
}

async fn page_news(locale: Locale, path: &str) -> Result<HttpResponse> {
    let s = pages::News::new(locale, path)
        .render()
        .map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_item(
    locale: Locale,
    path: &str,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Item::new(locale, path, &query.into_inner())
        .render()
        .map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(
    locale: Locale,
    path: &str,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Search::new(locale, path, &query.into_inner())
        .render()
        .map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
