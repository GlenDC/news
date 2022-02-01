use std::collections::BTreeMap;

use crate::site::l18n;

pub struct SiteLocales<'a> {
    name: String,
    repository: &'a str,
    locale: &'a str,
    path: &'a str,
    query: Option<PageQuery<'a>>,
    locales: BTreeMap<&'a str, LocaleInfo>,
    nav: NavLocales,
}

pub struct LocaleInfo {
    name: String,
    active: bool,
}

impl<'a> SiteLocales<'a> {
    pub fn new(
        locale: &'a str,
        path: &'a str,
        query: Option<BTreeMap<&'a str, &'a str>>,
    ) -> SiteLocales<'a> {
        let mut locales = BTreeMap::new();
        for site_locale in l18n::SUPPORTED_LOCALES {
            locales.insert(
                *site_locale,
                LocaleInfo {
                    name: l18n::txt(format!("site.locales.{}", site_locale).as_str(), locale),
                    active: (*site_locale) == locale,
                },
            );
        }
        SiteLocales {
            name: l18n::txt("site.name", locale),
            repository: "https://github.com/plabayo/news",
            locale,
            locales,
            path,
            query: query.map(|params| PageQuery { params }),
            nav: NavLocales {
                header: NavHeaderLocales {
                    news: l18n::txt("site.nav.header.news", locale),
                    past: l18n::txt("site.nav.header.past", locale),
                    comments: l18n::txt("site.nav.header.comments", locale),
                    ask: l18n::txt("site.nav.header.ask", locale),
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
    pub fn params_for(&self, path: &str, ignore: &str) -> BTreeMap<&str, &str> {
        let params_to_ignore: Vec<&str> = ignore.split('&').collect();
        let mut params = BTreeMap::new();
        if !params_to_ignore.contains(&"loc") {
            params.insert("loc", self.locale);
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

struct NavLocales {
    header: NavHeaderLocales,
    footer: NavFooterLocales,
}

struct NavHeaderLocales {
    news: String,
    past: String,
    comments: String,
    ask: String,
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

struct PageQuery<'a> {
    params: BTreeMap<&'a str, &'a str>,
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
        i18n: NotFoundLocales<'a>,
    }

    impl<'a> NotFound<'a> {
        pub fn new(locale: &'a str, path: &'a str, info: &'a SiteInfo) -> NotFound<'a> {
            NotFound {
                site_info: info,
                i18n: NotFoundLocales {
                    site: SiteLocales::new(locale, path, None),
                },
            }
        }
    }

    struct NotFoundLocales<'a> {
        site: SiteLocales<'a>,
    }

    #[derive(Template)]
    #[template(path = "pages/index.html")]
    pub struct News<'a> {
        site_info: &'a SiteInfo,
        i18n: NewsLocales<'a>,
    }

    impl<'a> News<'a> {
        pub fn new(locale: &'a str, path: &'a str, info: &'a SiteInfo) -> News<'a> {
            News {
                site_info: info,
                i18n: NewsLocales {
                    site: SiteLocales::new(locale, path, None),
                },
            }
        }
    }

    struct NewsLocales<'a> {
        site: SiteLocales<'a>,
    }

    #[derive(Template)]
    #[template(path = "pages/item.html")]
    pub struct Item<'a> {
        site_info: &'a SiteInfo,
        q: &'a str,
        i18n: ItemLocales<'a>,
    }

    struct ItemLocales<'a> {
        site: SiteLocales<'a>,
    }

    impl<'a> Item<'a> {
        pub fn new(
            locale: &'a str,
            path: &'a str,
            info: &'a SiteInfo,
            params: &'a BTreeMap<String, String>,
        ) -> Item<'a> {
            let q = match params.get("q") {
                Some(s) => s,
                None => "",
            };
            let mut query: BTreeMap<&'a str, &'a str> = BTreeMap::new();
            query.insert("q", q);
            Item {
                site_info: info,
                q,
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
        i18n: SearchLocales<'a>,
    }

    struct SearchLocales<'a> {
        site: SiteLocales<'a>,
    }

    impl<'a> Search<'a> {
        pub fn new(
            locale: &'a str,
            path: &'a str,
            info: &'a SiteInfo,
            params: &'a BTreeMap<String, String>,
        ) -> Search<'a> {
            let q = match params.get("q") {
                Some(s) => s,
                None => "",
            };
            let mut query: BTreeMap<&'a str, &'a str> = BTreeMap::new();
            query.insert("q", q);
            Search {
                site_info: info,
                q,
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
        i18n: SecurityLocales<'a>,
    }

    struct SecurityLocales<'a> {
        site: SiteLocales<'a>,
        intro: String,
        reports: String,
    }

    impl<'a> Security<'a> {
        pub fn new(locale: &'a str, path: &'a str, info: &'a SiteInfo) -> Security<'a> {
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
