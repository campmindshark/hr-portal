use log::info;
use rocket::{Request, Response, Rocket};
use rocket::fairing::{Fairing, Info, Kind};

pub struct RequestLogger;

impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Launch | Kind::Response,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        let config = rocket.config();
        let addr = format!("http://[{}]:{}", &config.address, &config.port);
        info!("Service is running on: {}", addr);
        info!("Currently running environment: {}", &config.environment);

        info!("Service has the following routes configured:");
        for route in rocket.routes() {
            if route.rank < 0 {
                info!("\t{} {}", route.method, route.uri);
            } else {
                info!("\t{} {} [{}]", route.method, route.uri, route.rank);
            }
        }
    }

    fn on_response(&self, request: &Request<'_>, response: &mut Response) {
        let method = request.method();
        let status = response.status();
        let uri = request.uri().to_string();

        if let Some(ref route) = request.route() {
            info!("{} {} (handler: {}) => {} {}", method, uri, route, status.code, status.reason)
        } else {
            info!("{} {} => {} {}", method, uri, status.code, status.reason)
        }
    }
}
