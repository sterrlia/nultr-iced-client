use serde::{Deserialize, Serialize};

use super::api::HttpRequest;

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

impl HttpRequest for LoginRequest {
    const ENDPOINT: &'static str = "login";
    const METHOD: reqwest::Method = reqwest::Method::POST;
}

pub struct GetMessagesRequest {}

pub struct Message {}
pub type GetMessagesResponse = Vec<Message>;

#[derive(Deserialize)]
pub enum Error {
    Deserialization,
    Response,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        tracing::error!("Deserialization error {value}");

        Error::Deserialization
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        tracing::error!("Request error {value}");

        Error::Response
    }
}

pub struct Session {
    pub token: String,
}

