use actix::prelude::*;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::{middleware, web, App, HttpServer};
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use handlebars::Handlebars;

use rustack::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let settings = settings::Settings::new().expect("error loading settings");
    let mailer = mailer::Mailer::new(&settings).start();
    let manager = ConnectionManager::<PgConnection>::new(settings.database.url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let secure = settings.http.secure.clone();
    let secret = settings.http.secret.clone();
    let templates_dir = settings.files.templates_dir.clone();
    let static_dir = settings.files.static_dir.clone();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("*.hbs", &templates_dir)
        .unwrap();

    let comrak_options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: Some("".into()),
            footnotes: true,
            ..ComrakExtensionOptions::default()
        },
        render: ComrakRenderOptions {
            escape: true,
            ..ComrakRenderOptions::default()
        },
        parse: ComrakParseOptions {
            smart: true,
            ..ComrakParseOptions::default()
        },
    };

    let settings_data = web::Data::new(settings);
    let mailer_data = web::Data::new(mailer);
    let comrak_data = web::Data::new(comrak_options);

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
            .wrap(
                CookieSession::private(secret.as_bytes())
                    .secure(secure)
                    .max_age(7 * 24 * 60 * 60)
                    .same_site(SameSite::Lax),
            )
            .service(fs::Files::new("/static", &static_dir))
            .configure(controllers::urls)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
