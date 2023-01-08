use std::env;

pub struct EnvVars {
    pub api_base_url: String,
    pub api_key_id: String,
    pub api_secret: String,
}

impl EnvVars {
    pub fn new() -> EnvVars {
        EnvVars {
            api_base_url: env::var("APCA_API_BASE_URL").expect("APCA_API_BASE_URL not found"),
            api_key_id: env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID not found"),
            api_secret: env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET not found"),
        }
    }


}

