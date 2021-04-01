use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod controllers;
mod mailer;
mod models;
mod schema;
mod settings;
mod markdown;
mod errors;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Load settings
    let settings = settings::Settings::new().expect("error loading settings");

    // Start the mailer actor.
    let mailer = mailer::Mailer::new(&settings).start();

    let pool = PgPoolOptions::new()
        .connect(&settings.database.url)
        .await
        .expect("error creating db pool");

    let static_dir = settings.files.static_dir.clone();

    // Markdown parser config.
    let comrak_options = markdown::get_markdown_options();

    let settings_data = web::Data::new(settings);
    let mailer_data = web::Data::new(mailer);
    let comrak_data = web::Data::new(comrak_options);

    // find out how to get url from name like flask url_for

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(settings_data.clone())
            .data(mailer_data.clone())
            .app_data(comrak_data.clone())
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::new(
                r#"%a %{X-Real-IP}i %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/static", &static_dir))
            .configure(controllers::urls)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
