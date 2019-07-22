mod controllers;
mod view_models;
mod services;

use actix_files as fs;
use actix_web::*;
use std::{
    thread,
    time::Duration,
    sync::mpsc
};
use serde::Deserialize;
use serde_json;

use crate::controllers::pages_controller::*;
use crate::services::twitch_service::*;

const STREAMERS_JSON : &'static str = include_str!("../streamers.json");

#[derive(Deserialize)]
struct Streamers {
    youtube: Vec<String>,
    twitch: Vec<String>
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let streamers_list : Streamers = serde_json::from_str(STREAMERS_JSON)?;
    let twitch_streamers_id_list = get_info_from_usernames(streamers_list.twitch)
        .expect("Could not get twitch streamers id's").get_ids();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            println!("Fetching info");

            get_online_channels(&twitch_streamers_id_list)
            .map(|i| dbg!(i));

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
                fs::Files::new("/resources", "../")
                    .show_files_listing()
            )
    )
        .bind("localhost:8080")?
        .run()
}
