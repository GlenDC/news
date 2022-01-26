use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};
use askama::Template;

use crate::site::state::SiteState;
use crate::site::templates::pages;

const DEFAULT_LOCALE: &'static str = "en";
const ALL_LOCALES: &'static [&'static str] = &[DEFAULT_LOCALE, "nl", "es"];

pub async fn page_home(data: web::Data<SiteState>) -> Result<HttpResponse> {
    // TODO: respect [cookie > user > browser > fallback] order instead of hard coding
    page_news(DEFAULT_LOCALE, DEFAULT_LOCALE, data).await
}

pub async fn page_home_with_locale_or_path(
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

pub async fn page_home_with_locale_and_path(
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

async fn page_unknown(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = pages::NotFound::new(locale, param_locale, &data.info)
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_news(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = pages::News::new(locale, param_locale, &data.info)
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
    let s = pages::Item::new(locale, param_locale, &data.info, &query.into_inner())
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
    let s = pages::Search::new(locale, param_locale, &data.info, &query.into_inner())
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_security(
    locale: &str,
    param_locale: &str,
    data: web::Data<SiteState>,
) -> Result<HttpResponse> {
    let s = pages::Security::new(locale, param_locale, &data.info)
        .render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
