use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub fn urls(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
pub fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "title": "Home"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}
