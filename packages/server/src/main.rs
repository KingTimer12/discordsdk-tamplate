use std::{env, io::Result};

use actix_web::{post, web, App, HttpServer};
use dotenv::dotenv;
use serde::Deserialize;

use crate::api::DiscordApi;

mod api;

#[derive(Deserialize)]
struct TokenReq {
    code: String,
}

#[post["/api/token"]]
async fn token(req: web::Json<TokenReq>) -> Result<String> {
    let url = env::var("VITE_DISCORD_API_BASE").expect("VITE_DISCORD_API_BASE must be set.");
    let mut discord_api = DiscordApi::new(&url, &req.code);
    let token = discord_api.get_token().await;
    Ok(token.unwrap().to_string())
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(token))
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
