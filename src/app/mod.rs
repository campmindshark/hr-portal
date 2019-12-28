use rocket::{get, routes, Route};

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
pub fn index() -> &'static str {
    "homepage"
}
