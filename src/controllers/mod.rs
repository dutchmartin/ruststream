pub mod pages_controller {


    use crate::view_models::Livestreamer::Twitch;
    use crate::services::twitch_service::StreamList;
    use crate::view_models::{
        StreamViewModel,
        Livestreamer
    };
    use askama::Template;
    use std::sync::Mutex;
    use actix_web::{
        Responder,
        HttpResponse,
        web
    };


    pub fn index(available_streams: web::Data<Mutex<StreamList>>) -> impl Responder {
        let mut title = "Nobody is Streaming rust programming".to_string();
        let mut streamer_username: String = "iamdutchmartin".to_string();

        let stream_list = available_streams.lock()
            .expect("Cannot read value");
        let featured_stream = match stream_list.streams.first() {
            Some(stream) => {
                title = String::from(stream.get_channel_name()).to_owned() + " is streaming";
                streamer_username = stream.get_channel_name();
                StreamViewModel {
                    title: &title,
                    live_streamer: Livestreamer::Twitch(&streamer_username)
                }
            }
            None => {
                StreamViewModel {
                    title: &title,
                    live_streamer: Livestreamer::None
                }
            }
        };

        match featured_stream.render() {
            Ok(template) => return HttpResponse::Ok().body(template),
            Err(_err) => return HttpResponse::NotImplemented().body("error".to_owned())
        }
    }

    pub fn streamers() -> impl Responder {
        let model = StreamViewModel { title: "To be implemented...", live_streamer: Livestreamer::None };
        match model.render() {
            Ok(template) => return HttpResponse::Ok().body(template),
            Err(_err) => return HttpResponse::NotImplemented().body("error".to_owned())
        }
    }
}