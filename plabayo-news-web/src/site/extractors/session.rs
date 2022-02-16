use actix_web::dev::Payload;
use actix_web::http::header::ACCEPT_LANGUAGE;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};

use crate::site::l18n::locales::Locale;

#[derive(Default)]
pub struct Session {
    headers: Headers,
}

impl Session {
    pub fn locale(&self) -> Locale {
        if let Some(locale) = self.headers.locale {
            return locale;
        }
        Locale::default()
    }
}

#[derive(Default)]
pub struct Headers {
    locale: Option<Locale>,
}

impl Headers {
    fn from_request(req: &HttpRequest) -> Headers {
        let locale = req
            .headers()
            .get(ACCEPT_LANGUAGE)
            .and_then(|hv| hv.to_str().ok())
            .unwrap_or_else(|| Locale::default().as_str())
            .split(",")
            .map(|language| {
                language
                    .trim()
                    .split("-")
                    .next()
                    .unwrap_or(language)
                    .split("_")
                    .next()
                    .unwrap_or(language)
            })
            .filter_map(|s| Locale::try_from(s).ok())
            .next();
        Headers { locale }
    }
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;
    type Config = SessionConfig;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = Headers::from_request(req);
        ready(Ok(Session { headers }))
    }
}

#[derive(Clone, Default)]
pub struct SessionConfig {}
