mod controllers;
mod view_models;

use actix_files as fs;
use actix_web::*;

use crate::controllers::pages_controller::*;

fn main() -> std::io::Result<()> {
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
