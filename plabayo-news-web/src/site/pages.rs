// Plabayo News
// Copyright (C) 2021  Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::BTreeMap;

use actix_web::dev::HttpServiceFactory;
use actix_web::{error::ErrorInternalServerError, web, HttpResponse, Result};
use askama::Template;

use crate::site::extractors::Session;
use crate::site::l18n::locales::Locale;
use crate::site::l18n::pages::static_response;
use crate::site::templates::pages;

pub fn factory() -> impl HttpServiceFactory + 'static {
    web::resource("/{resource:.*}").route(web::get().to(serve_page))
}

async fn serve_page(
    path: web::Path<(String,)>,
    query: web::Query<BTreeMap<String, String>>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
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
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_item(
    locale: Locale,
    path: &str,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Item::new(locale, path, &query.into_inner())
        .render()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page_search(
    locale: Locale,
    path: &str,
    query: web::Query<BTreeMap<String, String>>,
) -> Result<HttpResponse> {
    let s = pages::Search::new(locale, path, &query.into_inner())
        .render()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
