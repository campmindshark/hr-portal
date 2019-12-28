#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod app;
mod authentication;
mod db;
mod middleware;
mod status;

use log::info;
use rocket_contrib::serve::{Options, StaticFiles};

fn configure_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                // I may want this to be UTC...
                chrono::Local::now().format("[%+]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        // Turn off rocket's shitty logging
        .level_for("_", log::LevelFilter::Off)
        .level_for("launch", log::LevelFilter::Off)
        .level_for("launch_", log::LevelFilter::Off)
        .level_for("rocket::fairing::fairings", log::LevelFilter::Off)
        .level_for("rocket::rocket", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

fn version_dump() {
    info!("BUILD_PROFILE: {}", env!("BUILD_PROFILE"));
    info!("RUSTC_VERSION: {}", env!("RUSTC_VERSION"));
    info!("RUST_TARGET: {}", env!("RUST_TARGET"));

    if let Some(branch) = option_env!("GIT_BRANCH") {
        info!("GIT_BRANCH: {}", branch);
    }

    info!("GIT_DESCRIBE: {}", env!("GIT_DESCRIBE"));
    info!("GIT_DIRTY: {}", env!("GIT_DIRTY"));
    info!("GIT_REVISION: {}", env!("GIT_REVISION"));

    if let Some(branch) = option_env!("GIT_TAG") {
        info!("GIT_TAG: {}", branch);
    }
}

fn main() {
    dotenv::dotenv().ok();

    if let Err(err) = configure_logging() {
        println!("Couldn't setup logging! Aborting execution. Error was: {}", err);
        std::process::exit(1);
    }

    version_dump();

    let rocket = rocket::ignite()
                    .mount("/auth", authentication::routes())
                    .mount("/status", status::routes())
                    .mount("/", app::routes())
                    .mount("/", StaticFiles::new("./static", Options::None))
                    .attach(rocket_contrib::templates::Template::fairing())
                    .attach(middleware::RequestLogger);

    let env = rocket.config().environment.clone();
    rocket.attach(middleware::SecurityHeaders::new(env)).launch();
}
