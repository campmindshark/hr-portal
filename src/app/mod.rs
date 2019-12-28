use rocket::{get, routes, Route};
use rocket_contrib::templates::Template;
use serde::Serialize;

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[derive(Serialize)]
struct EmptyContext {
    parent: &'static str,
}

impl Default for EmptyContext {
    fn default() -> Self {
        EmptyContext { parent: "layout" }
    }
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &EmptyContext::default())
}
