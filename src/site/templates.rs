use std::collections::BTreeMap;

use crate::site::l18n;

pub struct SiteLocales {
    name: String,
    repository: String,
    locale: String,
    path: String,
    query: Option<PageQuery>,
    locales: BTreeMap<String, String>,
    nav: NavLocales,
}

impl SiteLocales {
    pub fn new(locale: &str, path: &str, query: Option<BTreeMap<String, String>>) -> SiteLocales {
        let mut locales = BTreeMap::new();
        for site_locale in l18n::SUPPORTED_LOCALES {
            locales.insert(String::from(*site_locale), l18n::txt(format!("site.locales.{}", site_locale).as_str(), locale));
        }
        SiteLocales {
            name: l18n::txt("site.name", locale),
            repository: String::from("https://github.com/plabayo/news"),
            locale: String::from(locale),
            locales: locales,
            path: String::from(path),
            query: query.map(|params| PageQuery{ params }),
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
                    locale: l18n::txt("site.nav.header.locale", locale),
                    select: l18n::txt("site.nav.header.select", locale),
                },
                footer: NavFooterLocales {
                    guidelines: l18n::txt("site.nav.footer.guidelines", locale),
                    faq: l18n::txt("site.nav.footer.faq", locale),
                    api: l18n::txt("site.nav.footer.api", locale),
                    security: l18n::txt("site.nav.footer.security", locale),
                    legal: l18n::txt("site.nav.footer.legal", locale),
                    contact: l18n::txt("site.nav.footer.contact", locale),
                    search: l18n::txt("site.nav.footer.search", locale),
                    build_info: l18n::txt("site.nav.footer.build_info", locale),
                },
            },
        }
    }
    
    pub fn params_for(&self, path: &str) -> BTreeMap<&str, &str> {
        let mut params = BTreeMap::new();
        params.insert("loc", self.locale.as_str());
        if self.path == path {
            if let Some(query) = self.query.as_ref() {
                for (key, value) in query.params.iter() {
                    params.insert(key.as_str(), value.as_str());
                }
            }
        }
        params
    }
    
    pub fn params_current(&self, with_locale: bool) -> BTreeMap<&str, &str> {
        let mut params = BTreeMap::new();
        if with_locale {
            params.insert("loc", self.locale.as_str());
        }
        if let Some(query) = self.query.as_ref() {
            for (key, value) in query.params.iter() {
                params.insert(key.as_str(), value.as_str());
            }
        }
        params
    }

    pub fn page_query_for(&self, path: &str) -> String {
        let params = self.params_for(path);
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
    locale: String,
    select: String,
}

struct NavFooterLocales {
    guidelines: String,
    faq: String,
    api: String,
    security: String,
    legal: String,
    contact: String,
    search: String,
    build_info: String,
}

struct PageQuery {
    params: BTreeMap<String, String>,
}

pub mod pages {
    use std::collections::BTreeMap;

    use askama::Template;

    use super::*;
    use crate::site::state::SiteInfo;

    #[derive(Template)]
    #[template(path = "pages/not_found.html")]
    pub struct NotFound<'a> {
        site_info: &'a SiteInfo,
        i18n: NotFoundLocales,
    }

    impl<'a> NotFound<'a> {
        pub fn new(locale: &str, path: &str, info: &'a SiteInfo) -> NotFound<'a> {
            NotFound {
                site_info: info,
                i18n: NotFoundLocales {
                    site: SiteLocales::new(locale, path, None),
                },
            }
        }
    }

    struct NotFoundLocales {
        site: SiteLocales,
    }

    #[derive(Template)]
    #[template(path = "pages/index.html")]
    pub struct News<'a> {
        site_info: &'a SiteInfo,
        i18n: NewsLocales,
    }

    impl<'a> News<'a> {
        pub fn new(locale: &str, path: &str, info: &'a SiteInfo) -> News<'a> {
            News {
                site_info: info,
                i18n: NewsLocales {
                    site: SiteLocales::new(locale, path, None),
                },
            }
        }
    }

    struct NewsLocales {
        site: SiteLocales,
    }

    #[derive(Template)]
    #[template(path = "pages/item.html")]
    pub struct Item<'a> {
        site_info: &'a SiteInfo,
        q: &'a str,
        i18n: ItemLocales,
    }

    struct ItemLocales {
        site: SiteLocales,
    }

    impl<'a> Item<'a> {
        pub fn new(
            locale: &str,
            path: &str,
            info: &'a SiteInfo,
            params: &'a BTreeMap<String, String>,
        ) -> Item<'a> {
            let q = match params.get("q") {
                Some(s) => &s,
                None => "",
            };
            let mut query: BTreeMap<String, String> = BTreeMap::new();
            query.insert(String::from("q"), String::from(q));
            Item {
                site_info: info,
                q: q,
                i18n: ItemLocales {
                    site: SiteLocales::new(locale, path, Some(query)),
                },
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/search.html")]
    pub struct Search<'a> {
        site_info: &'a SiteInfo,
        q: &'a str,
        i18n: SearchLocales,
    }

    struct SearchLocales {
        site: SiteLocales,
    }

    impl<'a> Search<'a> {
        pub fn new(
            locale: &str,
            path: &str,
            info: &'a SiteInfo,
            params: &'a BTreeMap<String, String>,
        ) -> Search<'a> {
            let q = match params.get("q") {
                Some(s) => &s,
                None => "",
            };
            let mut query: BTreeMap<String, String> = BTreeMap::new();
            query.insert(String::from("q"), String::from(q));
            Search {
                site_info: info,
                q: q,
                i18n: SearchLocales {
                    site: SiteLocales::new(locale, path, Some(query)),
                },
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/security.html", escape = "none")]
    pub struct Security<'a> {
        site_info: &'a SiteInfo,
        i18n: SecurityLocales,
    }

    struct SecurityLocales {
        site: SiteLocales,
        intro: String,
        reports: String,
    }

    impl<'a> Security<'a> {
        pub fn new(locale: &str, path: &str, info: &'a SiteInfo) -> Security<'a> {
            Security {
                site_info: info,
                i18n: SecurityLocales {
                    site: SiteLocales::new(locale, path, None),
                    intro: l18n::md("page.security.intro", locale),
                    reports: l18n::md("page.security.reports", locale),
                },
            }
        }
    }
}
