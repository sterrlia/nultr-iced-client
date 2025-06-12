use std::sync::Arc;

use iced::Task;
use tokio::sync::mpsc;

use crate::{
    http::{
        self,
        models::{ApiError, LoginRequest, LoginResponse},
    },
    ws::{self},
};

pub struct Service {
    pub http_client: Arc<http::api::Client>,
}

impl Default for Service {
    fn default() -> Self {
        let http_client = Arc::new(http::api::Client::default());

        Self { http_client }
    }
}

impl Service {}
