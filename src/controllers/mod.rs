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
        // TODO: implement a read from the mutex.
        let mut title = "Nobody is Streaming rust programming".to_string();
        let stream_list = available_streams.lock()
            .expect("Cannot read value");
        let featured_stream = match stream_list.streams.first() {
            Some(stream) => {
                title = String::from(stream.get_channel_name()).to_owned() + " is streaming";
                StreamViewModel {
                    title: &title,
                    live_streamer: Livestreamer::None
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
        let model = StreamViewModel { title: "John stream", live_streamer: Livestreamer::None };
        match model.render() {
            Ok(template) => return HttpResponse::Ok().body(template),
            Err(_err) => return HttpResponse::NotImplemented().body("error".to_owned())
        }
    }
}