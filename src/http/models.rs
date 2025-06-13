use chrono::NaiveDateTime;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::api::{HttpRequest, RequestError};

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

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ErrorResponse {
    InternalServerError,
    UserNotFound,
    AccessDenied,
    InvalidToken
}

impl HttpRequest<LoginResponse, ErrorResponse> for LoginRequest {
    const ENDPOINT: &'static str = "api/login";
    const METHOD: reqwest::Method = reqwest::Method::POST;
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Serialize)]
pub struct GetMessagesRequest {
    pub user_id: i32,
    #[serde(flatten)]
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

impl HttpRequest<GetMessagesResponse, ErrorResponse> for GetMessagesRequest {
    const ENDPOINT: &'static str = "api/get-messages";
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

impl HttpRequest<GetUsersResponse, ErrorResponse> for GetUsersRequest {
    const ENDPOINT: &'static str = "api/get-users";
    const METHOD: reqwest::Method = reqwest::Method::GET;
}

pub struct Session {
    pub token: String,
}

