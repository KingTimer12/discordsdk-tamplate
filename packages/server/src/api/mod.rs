use std::{collections::HashMap, env};

use json::JsonValue;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};

pub struct Credentials {
    client_id: String,
    client_secret: String,
}

impl Credentials {
    pub fn new() -> Self {
        let client_id = env::var("VITE_CLIENT_ID").expect("VITE_CLIENT_ID must be set.");
        let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set.");
        Self {
            client_id,
            client_secret,
        }
    }
}

pub struct DiscordApi {
    url: String,
    client: Client,
    credentials: Credentials,
    code: String,
}

impl DiscordApi {
    pub fn new(url: &str, code: &str) -> Self {
        let client_builder = ClientBuilder::new();
        let client = client_builder.build().unwrap();
        Self {
            url: url.to_string(),
            client,
            credentials: Credentials::new(),
            code: code.to_string(),
        }
    }

    pub async fn get_token(&mut self) -> Option<JsonValue> {
        let url = &self.url;
        let client_id = &self.credentials.client_id;
        let client_secret = &self.credentials.client_secret;
        let code = &self.code;
        let grant_type = &format!("authorization_code");

        let mut map = HashMap::new();
        map.insert("client_id", client_id);
        map.insert("client_secret", client_secret);
        map.insert("grant_type", grant_type);
        map.insert("code", code);

        let result = self
            .client
            .post(url)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .form(&map)
            .send()
            .await;
        if result.is_err() {
            return None;
        }
        let response = result.unwrap().text();
        let data = response.await.unwrap();
        Some(json::parse(&data).unwrap())
    }
}
