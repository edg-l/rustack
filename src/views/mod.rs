//! Views
//!
//! Define the app views here.
//!

pub use askama::Template;
use derive_more::Constructor;

#[derive(Template, Constructor)]
#[template(path = "index.html")]
pub struct Error<'a> {
    pub title: &'a str,
    pub code: i32,
    pub msg: &'a str,
}

#[derive(Template, Constructor)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub title: &'a str
}
