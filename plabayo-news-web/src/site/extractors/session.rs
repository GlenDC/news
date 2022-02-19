use std::collections::BTreeMap;
use std::time::SystemTime;

use actix_web::dev::Payload;
use actix_web::http::header::ACCEPT_LANGUAGE;
use actix_web::web::Query;
use actix_web::{Error, FromRequest, HttpRequest};
use anyhow::Result;
use futures::future::{ready, Ready};

use plabayo_news_data::models::{User, UserID, UserState};

use crate::site::l18n::locales::Locale;

#[derive(Default)]
pub struct Session {
    headers: Headers,
    user_ref: Option<UserReference>,
}

impl Session {
    pub fn locale(&self) -> Locale {
        if let Some(locale) = self
            .user()
            .as_ref()
            .and_then(|user| user.locale.as_ref())
            .and_then(|s| Locale::try_from(s.as_str()).ok())
        {
            return locale;
        }
        if let Some(locale) = self.headers.locale {
            return locale;
        }
        Locale::default()
    }

    pub fn user(&self) -> Option<User> {
        match self.user_ref.as_ref() {
            None => None,
            Some(user_ref) => user_ref.user().ok(), // TODO: log error perhaps!???!!
        }
    }
}

#[derive(Default)]
struct Headers {
    locale: Option<Locale>,
}

struct UserReference {
    user_id: UserID,
    last_fetch_time: SystemTime,
}

impl Headers {
    fn from_request(req: &HttpRequest) -> Headers {
        const HEADER_ACCEPT_LANGUAGE_VALUE_ANY: &str = "*";

        let locale = req
            .headers()
            .get(ACCEPT_LANGUAGE)
            .and_then(|hv| hv.to_str().ok())
            .unwrap_or(HEADER_ACCEPT_LANGUAGE_VALUE_ANY)
            .split(&[',', '-', '.', ';'])
            .map(|language| language.trim())
            .filter_map(|s| {
                if s == HEADER_ACCEPT_LANGUAGE_VALUE_ANY {
                    Some(Locale::default())
                } else {
                    Locale::try_from(s).ok()
                }
            })
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
        // TODO: delete this hack once we have a reg/login system
        let query_opt: Option<Query<BTreeMap<String, String>>> =
            Query::from_query(req.query_string()).ok();
        if let Some(user_id) = query_opt.as_ref().and_then(|q| q.get("id")) {
            if let Ok(user_id_num) = user_id.parse::<UserID>() {
                return ready(Ok(Session {
                    headers,
                    user_ref: Some(UserReference {
                        user_id: user_id_num,
                        last_fetch_time: SystemTime::UNIX_EPOCH,
                    }),
                }));
            }
        }
        ready(Ok(Session {
            headers,
            user_ref: None,
        }))
    }
}

impl UserReference {
    pub fn user(&self) -> Result<User> {
        Ok(User {
            id: self.user_id,
            state: UserState::Public,
            username: Some(format!("User#{}", self.user_id)),
            name: None,
            locale: None,
            location: None,
            create_time: self.last_fetch_time,
            last_login_time: self.last_fetch_time,
            karma: 42,
            about: None,
            items: vec![],
            ips: vec![],
            authentications: vec![],
            preferences: None,
        })
    }
}

#[derive(Clone, Default)]
pub struct SessionConfig {}
