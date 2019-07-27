pub mod twitch_service {
    use serde::{Deserialize, Serialize};
    use serde_json::value::Value;
    use std::error::Error;
    use reqwest::{
        Client,
        header::HeaderValue
    };

    const BASE_URL: &str = "https://api.twitch.tv/kraken/";

    #[derive(Deserialize, Debug)]
    pub struct TwitchUserList{
        users: Vec<TwitchUser>
    }

    impl TwitchUserList {
        pub fn get_ids(self) -> Vec<String> {
            self.users.iter().map(|i|i.id.to_owned()).collect()
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct TwitchUser {
        #[serde(rename="_id")]
        id: String,
        bio: Value,
        created_at: String,
        display_name: String,
        logo: String,
        name: String,
        updated_at: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Channel {
        _id: i64,
        broadcaster_language: String,
        created_at: String,
        display_name: String,
        followers: i64,
        game: String,
        language: String,
        logo: String,
        mature: bool,
        name: String,
        partner: bool,
        profile_banner_background_color: String,
        status: String,
        updated_at: String,
        url: String,
        views: i64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Preview {
        large: String,
        medium: String,
        small: String,
        template: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StreamList {
        pub streams: Vec<Stream>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Stream {
        _id: i64,
        average_fps: i64,
        channel: Channel,
        created_at: String,
        delay: i64,
        game: String,
        is_playlist: bool,
        preview: Preview,
        video_height: i64,
        viewers: i64,
    }
    impl Stream {
        pub fn get_channel_name(&self) -> String {
            self.channel.name.clone()
        }
    }

    pub fn get_online_channels(channel_ids: &Vec<String>) -> Result<StreamList, Box<dyn Error>> {
        let response : StreamList = Client::new()
            .get((BASE_URL.to_owned() + "streams/?channel=" + &channel_ids.join(",")).as_str())
            .header("Client-ID", get_client_id_header_value() )
            .header( reqwest::header::ACCEPT, "application/vnd.twitchtv.v5+json")
            .send()?
            .json()?;
        Ok(response)
    }

    pub fn get_info_from_usernames(usernames: Vec<String>) -> Result<TwitchUserList, Box<dyn Error>> {
        let response: TwitchUserList = Client::new()
            .get((BASE_URL.to_owned() + "users?login=" + &usernames.join(",").as_str()).as_str())
            .header("Client-ID", get_client_id_header_value() )
            .header( reqwest::header::ACCEPT, "application/vnd.twitchtv.v5+json")
            .send()?
            .json()?;
        Ok(response)
    }

    fn get_client_id() -> String {
        let default : String = "xxx".to_owned();

        let env_client_id = std::env::var("TWITCH_CLIENT_ID");

        let client_id : &String = match env_client_id {
            Ok(ref id) => id,
            Err(e) => {
                println!("Could not read ({})", e);
                &default
            }
        };
        client_id.to_string()
    }
    fn get_client_id_header_value() -> HeaderValue {
        match HeaderValue::from_str(&get_client_id()){
            Ok(val) => {
                val
            }
            _ => {
                panic!("Value from client id could not be converted.")
            }
        }
    }
}