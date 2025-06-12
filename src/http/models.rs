use chrono::NaiveDateTime;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::api::HttpRequest;

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub user_id: i32,
    pub token: String,
}

impl HttpRequest<LoginResponse, ApiError> for LoginRequest {
    const ENDPOINT: &'static str = "login";
    const METHOD: reqwest::Method = reqwest::Method::POST;
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Serialize)]
pub struct GetMessagesRequest {
    pub user_id: i32,
    pub pagination: Pagination
}

#[derive(Deserialize, Clone, Debug)]
pub struct MessageResponse {
    pub id: Uuid,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}
pub type GetMessagesResponse = Vec<MessageResponse>;

impl HttpRequest<GetMessagesResponse, ApiError> for GetMessagesRequest {
    const ENDPOINT: &'static str = "get-messages";
    const METHOD: reqwest::Method = reqwest::Method::GET;
}

#[derive(Serialize)]
pub struct GetUsersRequest {
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}
pub type GetUsersResponse = Vec<UserResponse>;

impl HttpRequest<GetUsersResponse, ApiError> for GetUsersRequest {
    const ENDPOINT: &'static str = "get-users";
    const METHOD: reqwest::Method = reqwest::Method::GET;
}

#[derive(Clone, Debug)]
pub enum ApiError {
    Deserialization,
    Http(StatusCode),
    Timeout,
    Connect,
    Redirect,
    Unknown,
    Decode
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        tracing::error!("Deserialization error {value}");

        ApiError::Deserialization
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        tracing::error!("Request error {value}");

        if let Some(status) = value.status() {
            ApiError::Http(status)
        } else if value.is_timeout() {
            ApiError::Timeout
        } else if value.is_connect() {
            ApiError::Connect
        } else if value.is_redirect() {
            ApiError::Redirect
        } else if value.is_decode() {
            ApiError::Decode
        } else {
            ApiError::Unknown
        }
    }
}

pub struct Session {
    pub token: String,
}
