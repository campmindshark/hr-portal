use lazy_static::lazy_static;
use rocket::{get, routes, Route};
use rocket::response::content;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
struct VersionResponse {
    build_profile: &'static str,
    rust_version: &'static str,
    rust_target: &'static str,

    git_describe: &'static str,
    git_dirty: bool,
    git_revision: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    git_branch: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    git_tag: Option<&'static str>,
}

lazy_static! {
    static ref CURRENT_VERSION: String = serde_json::to_string(
        &VersionResponse {
            build_profile: env!("BUILD_PROFILE"),
            rust_version: env!("RUSTC_VERSION"),
            rust_target: env!("RUST_TARGET"),

            git_describe: env!("GIT_DESCRIBE"),
            git_dirty: bool::from_str(&env!("GIT_DIRTY")).unwrap(),
            git_revision: env!("GIT_REVISION"),
            git_branch: option_env!("GIT_BRANCH"),
            git_tag: option_env!("GIT_TAG"),
        }
    ).unwrap();
}

pub fn routes() -> Vec<Route> {
    routes![version]
}

#[get("/version")]
pub fn version() -> content::Json<&'static str> {
    content::Json(&CURRENT_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_version_handler() {
        let rocket = rocket::ignite().mount("/", routes());
        let client = Client::new(rocket).expect("a valid rocket instance");

        let response = client.get("/version").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
