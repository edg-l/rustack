use actix_files as fs;
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::{middleware, App, HttpServer};
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use rustack::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();


    HttpServer::new(move || {
        let settings = settings::Settings::new().expect("error loading settings");
        let mailer = mailer::Mailer::new(&settings);
        let manager = ConnectionManager::<PgConnection>::new(settings.database.url.clone());
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let secure = settings.http.secure.clone();
        let secret = settings.http.secret.clone();

        App::new()
            .data(pool.clone())
            .data(settings)
            .data(mailer)
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
            .service(fs::Files::new("/static", "./static"))
            .configure(controllers::urls)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
