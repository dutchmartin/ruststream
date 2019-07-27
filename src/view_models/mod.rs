use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct StreamViewModel<'a> {
   pub title: &'a str,
   pub live_streamer: Livestreamer<'a>

}

pub enum Livestreamer<'a> {
   Twitch(&'a str),
   Youtube(&'a str),
   None
}

#[derive(Template)]
#[template(path = "streamers.html")]
pub struct StreamersViewModel{
   pub youtube_streamers : Vec<StreamerInformationViewModel>,
   pub twitch_streamers : Vec<StreamerInformationViewModel>
}

pub struct StreamerInformationViewModel {
   pub name : String,
   pub username : String
}