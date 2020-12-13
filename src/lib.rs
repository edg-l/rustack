use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
pub mod controllers;
pub mod mailer;
pub mod models;
pub mod schema;
pub mod settings;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
