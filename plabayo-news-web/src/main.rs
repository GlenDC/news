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

// use actix_web::{middleware, web, App, HttpServer};
use anyhow::{Context, Result};
use axum::{handler::Handler, routing::get, Router};
use structopt::StructOpt;
use tower_http::trace::TraceLayer;

// use plabayo_news_web::site::middleware as pn_middleware;
// use plabayo_news_web::site::state::AppState;
// use plabayo_news_web::site::{assets, pages};

use plabayo_news_web::site::assets;

#[derive(StructOpt, Debug)]
#[structopt(name = "plabayo-news-web")]
struct Opt {
    /// enable debugging features such as the logger
    #[structopt(short, long)]
    debug: bool,

    /// interface to bind to
    #[structopt(short, long, default_value = "127.0.0.1:8080")]
    interface: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        if opt.debug {
            std::env::set_var("RUST_LOG", "plabayo_news_web=debug,tower_http=debug")
        } else {
            std::env::set_var("RUST_LOG", "plabayo_news_web=warn,tower_http=warn")
        }
    }
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(assets::ROUTE, assets::handler.into_service())
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    let socket_addr = opt.interface.parse().with_context(|| {
        format!(
            "parse Plabayo News HTTPServer socket address from interface: {}",
            opt.interface
        )
    })?;
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .with_context(|| {
            format!(
                "run Plabayo News HTTPServer on interface: {}",
                opt.interface
            )
        })?;

    Ok(())
}

// #[actix_web::main]
// async fn main() -> Result<()> {
//     let opt = Opt::from_args();

//     if opt.debug {
//         std::env::set_var("RUST_LOG", "actix_web=info");
//     } else {
//         std::env::set_var("RUST_LOG", "actix_web=error");
//     }
//     env_logger::init();

//     // create app state used by all routes
//     let state = web::Data::new(AppState::new());

//     // start http server
//     HttpServer::new(move || {
//         App::new()
//             .app_data(state.clone())
//             .wrap(middleware::Logger::default())
//             .wrap(middleware::Compress::default())
//             .wrap(pn_middleware::Cache::default())
//             .wrap(pn_middleware::SiteInfo::default())
//             .wrap(middleware::NormalizePath::new(
//                 middleware::normalize::TrailingSlash::Trim,
//             ))
//             .service(assets::factory())
//             .service(pages::factory())
//     })
//     .bind(&opt.interface)
//     .with_context(|| {
//         format!(
//             "bind Plabayo News HTTPServer to interface: {}",
//             opt.interface
//         )
//     })?
//     .run()
//     .await?;

//     Ok(())
// }
