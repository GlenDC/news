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

use crate::site::l18n::locales::Locale;

pub struct PageState<'a> {
    pub locale: Locale,
    pub path: &'a str,
    pub query: Option<PageQuery<'a>>,
}

impl<'a> PageState<'a> {
    pub fn new(
        locale: Locale,
        path: &'a str,
        query: Option<BTreeMap<&'a str, &'a str>>,
    ) -> PageState<'a> {
        PageState {
            locale,
            path,
            query: query.map(|params| PageQuery { params }),
        }
    }

    pub fn params_for(&self, path: &str, ignore: &str) -> BTreeMap<&str, &str> {
        let params_to_ignore: Vec<&str> = ignore.split('&').collect();
        let mut params = BTreeMap::new();
        if !params_to_ignore.contains(&"loc") {
            params.insert("loc", self.locale.as_str());
        }
        if self.path == path {
            if let Some(query) = self.query.as_ref() {
                for (key, value) in query
                    .params
                    .iter()
                    .filter(|(k, _)| !params_to_ignore.contains(k))
                {
                    params.insert(key, value);
                }
            }
        }
        params
    }

    pub fn params_current(&self, ignore: &str) -> BTreeMap<&str, &str> {
        self.params_for(self.path, ignore)
    }

    pub fn page_query_for(&self, path: &str, ignore: &str) -> String {
        let params = self.params_for(path, ignore);
        let mut params_iter = params.iter();
        let mut s = match params_iter.next() {
            None => return String::from(""),
            Some((key, value)) => format!("?{}={}", key, value),
        };
        for (key, value) in params_iter {
            s.push_str(format!("?{}={}", key, value).as_str());
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

pub struct PageQuery<'a> {
    params: BTreeMap<&'a str, &'a str>,
}

pub mod pages {
    use std::collections::BTreeMap;

    use askama::Template;

    use super::*;
    use crate::site::{SiteInfo, SITE_INFO};

    #[derive(Template)]
    #[template(path = "pages/index.html")]
    pub struct News<'a> {
        site_info: &'a SiteInfo,
        page: PageState<'a>,
    }

    impl<'a> News<'a> {
        pub fn new(locale: Locale, path: &'a str) -> News<'a> {
            let page = PageState::new(locale, path, None);
            News {
                site_info: &SITE_INFO,
                page,
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/item.html")]
    pub struct Item<'a> {
        site_info: &'a SiteInfo,
        q: &'a str,
        page: PageState<'a>,
    }

    impl<'a> Item<'a> {
        pub fn new(
            locale: Locale,
            path: &'a str,
            params: &'a BTreeMap<String, String>,
        ) -> Item<'a> {
            let q = match params.get("q") {
                Some(s) => s,
                None => "",
            };
            let mut query: BTreeMap<&'a str, &'a str> = BTreeMap::new();
            query.insert("q", q);
            let page = PageState::new(locale, path, Some(query));
            Item {
                site_info: &SITE_INFO,
                q,
                page,
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/search.html")]
    pub struct Search<'a> {
        site_info: &'a SiteInfo,
        q: &'a str,
        page: PageState<'a>,
    }

    impl<'a> Search<'a> {
        pub fn new(
            locale: Locale,
            path: &'a str,
            params: &'a BTreeMap<String, String>,
        ) -> Search<'a> {
            let q = match params.get("q") {
                Some(s) => s,
                None => "",
            };
            let mut query: BTreeMap<&'a str, &'a str> = BTreeMap::new();
            query.insert("q", q);
            let page = PageState::new(locale, path, Some(query));
            Search {
                site_info: &SITE_INFO,
                q,
                page,
            }
        }
    }
}
