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
use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};

use plabayo_news_data::models::User;

use crate::site::extractors::Session;
use crate::site::l18n::locales::Locale;
use crate::site::l18n::pages::{
    static_response, ContentIndex, ContentItem, ContentSearch, PageIndex, PageItem, PageSearch,
};

pub fn factory() -> impl HttpServiceFactory + 'static {
    web::resource("/{resource:.*}").route(web::get().to(serve_page))
}

pub struct PageState {
    pub locale: Locale,
    pub path: String,
    pub query: BTreeMap<String, String>,
    pub gen_date_time: DateTime<Utc>,
    pub user: Option<User>,
}

impl PageState {
    pub fn new(
        locale: Locale,
        path: String,
        query: BTreeMap<String, String>,
        user: Option<User>,
    ) -> PageState {
        PageState {
            locale,
            path,
            query,
            gen_date_time: chrono::offset::Utc::now(),
            user,
        }
    }

    pub fn params_for(&self, path: &str, ignore: &str) -> BTreeMap<&str, &str> {
        let params_to_ignore: Vec<&str> = ignore.split('&').collect();
        let mut params: BTreeMap<&str, &str> = BTreeMap::new();
        if self.path == path {
            for (key, value) in self
                .query
                .iter()
                .filter(|(k, _)| !params_to_ignore.contains(&k.as_str()))
            {
                params.insert(key, value);
            }
        }
        params
    }

    pub fn params_current(&self, ignore: &str) -> BTreeMap<&str, &str> {
        self.params_for(self.path.as_str(), ignore)
    }

    pub fn page_query_for(&self, path: &str, ignore: &str) -> String {
        let params = self.params_for(path, ignore);
        let mut params_iter = params.iter();
        let mut s = match params_iter.next() {
            None => return String::from(""),
            Some((key, value)) => format!("?{}={}", key, value),
        };
        for (key, value) in params_iter {
            s.push_str(format!("&{}={}", key, value).as_str());
        }
        s
    }

    pub fn class_nav_button_for(&self, path: &str) -> &str {
        if self.path == path {
            "selected"
        } else {
            "unselected"
        }
    }
}

async fn serve_page(
    path: web::Path<(String,)>,
    query: web::Query<BTreeMap<String, String>>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
    let user = session.user();
    let path = path.into_inner().0.to_lowercase();
    let query = query.into_inner();

    match path.as_str() {
        "" | "home" | "index" => PageIndex::new_response(
            PageState::new(locale, "/".to_string(), query, user),
            ContentIndex {},
        ),
        "new" | "news" => PageIndex::new_response(
            PageState::new(locale, "/news".to_string(), query, user),
            ContentIndex {},
        ),
        "search" => {
            let content = ContentSearch { q: "".to_string() };
            PageSearch::new_response(
                PageState::new(locale, "/search".to_string(), query, user),
                content,
            )
        }
        "item" => {
            let content = ContentItem { q: "".to_string() };
            PageItem::new_response(
                PageState::new(locale, "/item".to_string(), query, user),
                content,
            )
        }
        endpoint => static_response(
            path.as_str(),
            PageState::new(locale, format!("/{}", path), query, user),
        ),
    }
}
