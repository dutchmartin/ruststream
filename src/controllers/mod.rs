pub mod pages_controller {

    use crate::view_models::StreamViewModel;
    use askama::Template;
    use actix_web::HttpResponse;
    use actix_web::Responder;

    pub fn index() -> impl Responder {
        let model = StreamViewModel { title: "John stream" };
        match model.render() {
            Ok(template) => return HttpResponse::Ok().body(template),
            Err(_err) => return HttpResponse::NotImplemented().body("error".to_owned())
        }
    }
    pub fn streamers() ->impl Responder {
        let model = StreamViewModel { title: "John stream" };
        match model.render() {
            Ok(template) => return HttpResponse::Ok().body(template),
            Err(_err) => return HttpResponse::NotImplemented().body("error".to_owned())
        }
    }
}