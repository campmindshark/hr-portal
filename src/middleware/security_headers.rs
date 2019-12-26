use rocket::{Request, Response};
use rocket::config::Environment;
use rocket::fairing::{Fairing, Info, Kind};

pub struct SecurityHeaders {
    environment: Environment,
}

impl SecurityHeaders {
    pub fn new(env: Environment) -> Self {
        Self {
            environment: env,
        }
    }
}

impl Fairing for SecurityHeaders {
    fn info(&self) -> Info {
        Info {
            name: "Security Headers",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_raw_header("Referrer-Policy", "strict-origin");
        response.set_raw_header("X-Content-Type-Options", "nosniff");
        response.set_raw_header("X-Frame-Options", "SAMEORIGIN");
        response.set_raw_header("X-XSS-Protection", "1; mode=block");

        // Not really a security header, most of this is a private API, if I host this through a
        // proxy which is the most likely deployment scenario the responses should absolutely not
        // be cached. In the event I start hosting static files I may want to revisit this and
        // set this based on the path or conditionally if its not present or something. This is a
        // sane default for now.
        response.set_raw_header("Cache-Control", "no-cache, no-store, max-age=0");

        // It's never a good idea to expose information about the service you're running. I
        // understand why Rocket wants to set this header but its frankly a dangerous exposure.
        response.remove_header("Server");

        // In non-development environments I'm going to enforce the use of HTTPS and their best
        // practices. Anything else will explicitly require modifications to this source and be
        // unsupported.
        if self.environment != Environment::Development {
            response.set_raw_header("Strict-Transport-Security", "max-age=31536000; includeSubDomains;");
            // TODO: Do I want a reporting URI for this? It would be universal...
            response.set_raw_header("Expect-CT", "enforce, max-age=86400");
        }

        // TODO: CORS, CSP, feature policy
    }
}
