use actix_web::{HttpRequest, HttpResponse, get, web};
use crate::errors::*;
use crate::views::*;

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
