use actix_web::{HttpResponse, get, web};
use sitewriter::{Sitemap, Url};
use crate::errors::*;
use crate::views::*;
use crate::settings::Settings;

pub fn urls(cfg: &mut web::ServiceConfig) {
    // Add your endpoints here:
    cfg.service(index);
}

#[get("/")]
pub async fn index() -> AppResponse {
    let template = Index::new("Home");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/sitemap.xml")]
pub async fn sitemap(settings: web::Data<Settings>) -> AppResponse {
    let mut sitemap = Sitemap::new();

    let base_url = &settings.http.url;

    // Add urls for the sitemap here
    let index_url = base_url.join("/").unwrap();
    sitemap.urls.push(Url::new(index_url.as_str()));

    Ok(HttpResponse::Ok().content_type("text/xml").body(sitemap.into_str()))
}
