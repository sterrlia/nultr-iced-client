use serde::{Deserialize, Serialize};
use url::Url;

use crate::config;

use super::models::Session;

pub struct Client {
    client: reqwest::Client,
    base_url: Url,
}

impl Default for Client {
    fn default() -> Self {
        let client = reqwest::Client::new();
        let base_url = config::get_variables().http_url.clone();

        Self { client, base_url }
    }
}

impl Client {
    async fn get_raw_request_result<I>(
        &self,
        request: I,
        session: Option<Session>,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        I: HttpRequest + Serialize,
    {
        let endpoint_url = I::get_url(self.base_url.clone());
        let method = I::METHOD;

        let request_builder = self
            .client
            .request(method.clone(), endpoint_url)
            .json(&request);

        let request_builder = match method {
            reqwest::Method::GET => request_builder.query(&request),
            _ => request_builder.json(&request),
        };

        let request_builder = if let Some(session) = session {
            request_builder.bearer_auth(session.token)
        } else {
            request_builder
        };

        request_builder.send().await
    }

    pub async fn request<I, O, E>(&self, request: I, session: Option<Session>) -> Result<O, E>
    where
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de> + From<reqwest::Error> + From<serde_json::Error>,
        I: HttpRequest + Serialize,
    {
        let response = self.get_raw_request_result(request, session).await?;

        let response_body = response.text().await?;
        let result: O = serde_json::from_str(response_body.as_ref())?;
        Ok(result)
    }
}

pub trait HttpRequest {
    const ENDPOINT: &'static str;
    const METHOD: reqwest::Method;

    fn get_url(base_url: Url) -> Url {
        base_url.join(Self::ENDPOINT).unwrap()
    }
}
