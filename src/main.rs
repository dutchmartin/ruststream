mod controllers;
mod view_models;
mod services;

use actix_files as fs;
use actix_web::*;
use std::{
    thread,
    time::Duration,
    vec::Vec,
    ops::DerefMut,
    sync::Mutex,
    env::var
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
    dotenv::dotenv();
    let streamers_list : Streamers = serde_json::from_str(STREAMERS_JSON)?;
    let twitch_streamers_id_list = get_info_from_usernames(streamers_list.twitch)
        .expect("Could not get twitch streamers id's").get_ids();

    let twitch_streamers_list = web::Data::new(Mutex::new(
        StreamList {
            streams: Vec::new()
        }
    ));

    let i = twitch_streamers_list.clone();
    thread::spawn(move || {
        loop {
            println!("Fetching info");

            let updated_streamer_list = get_online_channels(&twitch_streamers_id_list)
            .expect("could not fetch value");
            *(&mut i
                .lock()
                .expect("could not lock mutex")).deref_mut() = updated_streamer_list;
            // Sleep one minute.
            thread::sleep(Duration::from_millis(60000));
        }
    });

    HttpServer::new(move || App::new()
        .register_data(twitch_streamers_list.clone())
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
        .bind(("localhost:".to_string() + &var("PORT").unwrap().to_string()).as_str())?
        .run()
}
