mod controllers;
mod view_models;
mod services;

use actix_files as fs;
use actix_web::*;
use std::thread;
use std::time::Duration;

use crate::controllers::pages_controller::*;
use crate::services::twitch_service;

fn main() -> std::io::Result<()> {
    thread::spawn(|| {
        loop {
            println!("Fetching info");
            twitch_service::is_channel_online("nietrickbroers");
            thread::sleep(Duration::from_millis(60000));
        }
    });

    HttpServer::new(|| App::new()
            .service(
                web::resource("/",).to(index)
            )
            .service(
                web::resource("/streamers",).to(streamers)
            )
            .service(
                fs::Files::new("/resources", "./resources")
                    .show_files_listing()
            )
    )
        .bind("localhost:8080")?
        .run()
}
