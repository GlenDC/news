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
use std::sync::Arc;

use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};

use plabayo_news_data::models::User;

use crate::site::extractors::Session;
use crate::site::l18n::locales::Locale;
use crate::site::l18n::pages::models::{ContentItem, ContentItems, ContentSearch, Item};
use crate::site::l18n::pages::{static_response, PageItem, PageItems, PageSearch};
use crate::site::state::AppState;

//---------------------------------------
// Actix Web Factory
//---------------------------------------

pub fn factory() -> impl HttpServiceFactory + 'static {
    web::resource("/{resource:.*}").route(web::get().to(serve_page))
}

//---------------------------------------
// Page State
//---------------------------------------

pub struct PageState {
    pub locale: Locale,
    pub path: String,
    pub query: BTreeMap<String, String>,
    pub gen_date_time: DateTime<Utc>,
    pub user: Option<User>,
}

// TODO: clean up this mess, so we can use cleanly in html templates,
//  e.g. instead of params_for, just provide a `query(&str)`,
//  which will keep all queries there are already on path,
//  and overwrite/add the (new) ones given.

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

//---------------------------------------
// Serve Definitions
//---------------------------------------

async fn serve_page(
    path: web::Path<(String,)>,
    query: web::Query<BTreeMap<String, String>>,
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let path = path.into_inner().0.to_lowercase();
    let query = query.into_inner();
    let app_state = app_state.into_inner();

    match path.as_str() {
        "" | "index" => serve_news_ranked("/", query, app_state, session).await,
        "news" => serve_news_ranked("/news", query, app_state, session).await,
        "search" => serve_search("/search", query, session).await,
        "item" => serve_item("/item", query, session).await,
        _ => serve_static(path.as_str(), query, session),
    }
}

async fn serve_news_ranked(
    path: &str,
    query: BTreeMap<String, String>,
    app_state: Arc<AppState>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
    let user = session.user();

    let content = ContentItems {
        items: app_state
            .db
            .get_news_ranked()
            .await
            .into_iter()
            .map(|data| Item::from_data(data))
            .collect(),
    };

    let page_state = PageState::new(locale, path.to_string(), query, user);

    PageItems::new_response(page_state, content)
}

async fn serve_search(
    path: &str,
    query: BTreeMap<String, String>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
    let user = session.user();

    // TODO: sanitize?!
    let q = query.get("q").map(|s| s.as_str()).unwrap_or("").to_string();

    let content = ContentSearch { q };

    let page_state = PageState::new(locale, path.to_string(), query, user);

    PageSearch::new_response(page_state, content)
}

async fn serve_item(
    path: &str,
    query: BTreeMap<String, String>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
    let user = session.user();

    // TODO: sanitize?!
    let q = query.get("q").map(|s| s.as_str()).unwrap_or("").to_string();

    let content = ContentItem { q };

    let page_state = PageState::new(locale, path.to_string(), query, user);

    PageItem::new_response(page_state, content)
}

fn serve_static(
    endpoint: &str,
    query: BTreeMap<String, String>,
    session: Session,
) -> Result<HttpResponse> {
    let locale = session.locale();
    let user = session.user();

    let page_state = PageState::new(locale, format!("/{}", endpoint), query, user);

    static_response(endpoint, page_state)
}

// TODO(2): provide data source trait in /data package,
// and use it within here to start to get data,
// so we are working towards that structure already,
// probably will require plenty of iterations on its own right
