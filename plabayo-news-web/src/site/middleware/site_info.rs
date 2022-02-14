use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::Future;
use lazy_static::lazy_static;

use crate::site::SITE_INFO;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Default)]
pub struct SiteInfo;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for SiteInfo
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SiteInfoMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SiteInfoMiddleware { service })
    }
}

pub struct SiteInfoMiddleware<S> {
    service: S,
}

impl<S, B> Service for SiteInfoMiddleware<S>
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
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let headers_mut = res.headers_mut();
            headers_mut.insert(
                HeaderName::from_static(HEADER_BUILD_VERSION),
                HeaderValue::from_static(HEADER_BUILD_VERSION_VALUE.as_str()),
            );
            headers_mut.insert(
                HeaderName::from_static(HEADER_BUILD_DATE),
                HeaderValue::from_static(HEADER_BUILD_DATE_VALUE.as_str()),
            );

            Ok(res)
        })
    }
}

const HEADER_BUILD_VERSION: &str = "x-plabayo-news-build-version";
const HEADER_BUILD_DATE: &str = "x-plabayo-news-build-date";

lazy_static! {
    static ref HEADER_BUILD_VERSION_VALUE: String =
        format!("v{}-{}", SITE_INFO.build_semver, SITE_INFO.git_sha_short);
    static ref HEADER_BUILD_DATE_VALUE: String = SITE_INFO.build_date.to_string();
}
