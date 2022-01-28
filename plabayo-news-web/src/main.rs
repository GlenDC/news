use actix_web::{middleware, App, HttpServer};

use plabayo_news_web::site::state::SiteState;
use plabayo_news_web::site::{assets, pages};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(SiteState::new())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .wrap(middleware::Compress::default())
            .service(assets::factory())
            .service(pages::factory())
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
