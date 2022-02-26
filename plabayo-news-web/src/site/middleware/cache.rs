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

use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{CacheControl, CacheDirective, Header, IntoHeaderValue};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::Future;

use crate::site::l18n::pages;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Default)]
pub struct Cache;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Cache
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheMiddleware { service })
    }
}

pub struct CacheMiddleware<S> {
    service: S,
}

impl<S, B> Service for CacheMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let cache_control_directive =
            IntoHeaderValue::try_into(if let Ok(cc) = CacheControl::parse(&req) {
                if cc.iter().any(|dir| dir == &CacheDirective::NoCache) {
                    CacheControl(vec![CacheDirective::NoCache])
                } else if let Some(CacheDirective::MaxAge(age)) = cc
                    .iter()
                    .find(|dir| matches!(dir, CacheDirective::MaxAge(_)))
                {
                    if *age == 0 {
                        CacheControl(vec![CacheDirective::NoCache])
                    } else {
                        CacheControl(vec![
                            CacheDirective::MaxAge(std::cmp::min(*age, 86400)), // 24h (or less)
                            CacheDirective::Public,
                        ])
                    }
                } else {
                    get_cache_control_directive_for_path(req.path())
                }
            } else {
                get_cache_control_directive_for_path(req.path())
            })
            .unwrap();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut()
                .insert(CacheControl::name(), cache_control_directive);
            Ok(res)
        })
    }
}

fn get_cache_control_directive_for_path(path: &str) -> CacheControl {
    CacheControl(match path.split('/').nth(1) {
        None => vec![CacheDirective::NoCache],
        Some(root) => {
            let max_age = pages::page_max_cache_age_sec(root);
            if max_age > 0 {
                vec![CacheDirective::MaxAge(max_age), CacheDirective::Public]
            } else {
                vec![CacheDirective::NoCache]
            }
        }
    })
}
