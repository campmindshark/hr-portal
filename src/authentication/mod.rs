use rocket::{post, routes, Route};

pub fn routes() -> Vec<Route> {
    routes![login]
}

#[post("/login")]
pub fn login() -> String {
    "todo".to_string()
}
