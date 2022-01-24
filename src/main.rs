use actix_web::{middleware, web, App, HttpServer};

use plabayo_news::site::{assets, pages};
use plabayo_news::site::state::SiteState;

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
            // TODO: replace with factory which will handle one function to rule them all
            .service(web::resource("/").route(web::get().to(pages::page_home)))
            .service(
                web::resource("/{locale_or_page}")
                    .route(web::get().to(pages::page_home_with_locale_or_path)),
            )
            .service(
                web::resource("/{locale}/{page}{tail:.*}")
                    .route(web::get().to(pages::page_home_with_locale_and_path)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
