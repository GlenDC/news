use crate::site::l18n;

pub struct SiteLocales {
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

pub mod pages {
    use std::collections::HashMap;

    use askama::Template;

    use super::*;
    use crate::site::state::SiteInfo;

    #[derive(Template)]
    #[template(path = "pages/not_found.html")]
    pub struct NotFound {
        site_info: SiteInfo,
        i18n: NotFoundLocales,
    }

    impl NotFound {
        pub fn new(locale: &str, param_locale: &str, info: SiteInfo) -> NotFound {
            NotFound {
                site_info: info,
                i18n: NotFoundLocales {
                    site: SiteLocales::new(locale, param_locale),
                },
            }
        }
    }

    struct NotFoundLocales {
        site: SiteLocales,
    }

    #[derive(Template)]
    #[template(path = "pages/index.html")]
    pub struct News {
        site_info: SiteInfo,
        i18n: NewsLocales,
    }

    impl News {
        pub fn new(locale: &str, param_locale: &str, info: SiteInfo) -> News {
            News {
                site_info: info,
                i18n: NewsLocales {
                    site: SiteLocales::new(locale, param_locale),
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
        site_info: SiteInfo,
        q: &'a str,
        i18n: ItemLocales,
    }

    struct ItemLocales {
        site: SiteLocales,
    }

    impl<'a> Item<'a> {
        pub fn new(locale: &str, param_locale: &str, info: SiteInfo, params: &'a HashMap<String, String>) -> Item<'a> {
            let q = match params.get("q") {
                Some(s) => &s,
                None => "",
            };
            Item {
                site_info: info,
                q: q,
                i18n: ItemLocales {
                    site: SiteLocales::new(locale, param_locale),
                },
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/search.html")]
    pub struct Search<'a> {
        site_info: SiteInfo,
        q: &'a str,
        i18n: SearchLocales,
    }

    struct SearchLocales {
        site: SiteLocales,
    }

    impl<'a> Search<'a> {
        pub fn new(locale: &str, param_locale: &str, info: SiteInfo, params: &'a HashMap<String, String>) -> Search<'a> {
            let q = match params.get("q") {
                Some(s) => &s,
                None => "",
            };
            Search {
                site_info: info,
                q: q,
                i18n: SearchLocales {
                    site: SiteLocales::new(locale, param_locale),
                },
            }
        }
    }

    #[derive(Template)]
    #[template(path = "pages/security.html", escape = "none")]
    pub struct Security {
        site_info: SiteInfo,
        i18n: SecurityLocales,
    }

    struct SecurityLocales {
        site: SiteLocales,
        intro: String,
        reports: String,
    }

    impl Security {
        pub fn new(locale: &str, param_locale: &str, info: SiteInfo) -> Security {
            Security {
                site_info: info,
                i18n: SecurityLocales {
                    site: SiteLocales::new(locale, param_locale),
                    intro: l18n::md("page.security.intro", locale),
                    reports: l18n::md("page.security.reports", locale),
                },
            }
        }
    }
}
