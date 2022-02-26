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

use actix_web::{middleware, web, App, HttpServer};
use anyhow::{Context, Result};
use structopt::StructOpt;

use plabayo_news_web::site::middleware as pn_middleware;
use plabayo_news_web::site::state::AppState;
use plabayo_news_web::site::{assets, pages};

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

#[actix_web::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.debug {
        std::env::set_var("RUST_LOG", "actix_web=info");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=error");
    }
    env_logger::init();

    // create app state used by all routes
    let state = web::Data::new(AppState::new());

    // start http server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(pn_middleware::Cache::default())
            .wrap(pn_middleware::SiteInfo::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .service(assets::factory())
            .service(pages::factory())
    })
    .bind(&opt.interface)
    .with_context(|| {
        format!(
            "bind Plabayo News HTTPServer to interface: {}",
            opt.interface
        )
    })?
    .run()
    .await?;

    Ok(())
}
