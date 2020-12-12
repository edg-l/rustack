use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
pub mod models;
pub mod controllers;
pub mod mailer;
pub mod settings;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
