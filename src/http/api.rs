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
    pub async fn request<I, O, E>(&self, request: I, session: Option<Session>) -> Result<O, E>
    where
        O: for<'de> Deserialize<'de>,
        E: From<reqwest::Error> + From<serde_json::Error>,
        I: HttpRequest<O, E> + Serialize,
    {
        request
            .perform(self.client.clone(), self.base_url.clone(), session)
            .await
    }
}

pub trait HttpRequest<O, E>
where
    O: for<'de> Deserialize<'de>,
    E: From<reqwest::Error> + From<serde_json::Error>,
    Self: Sized + Serialize,
{
    const ENDPOINT: &'static str;
    const METHOD: reqwest::Method;

    fn get_url(base_url: Url) -> Url {
        base_url.join(Self::ENDPOINT).unwrap()
    }

    async fn perform(
        &self,
        client: reqwest::Client,
        base_url: Url,
        session: Option<Session>,
    ) -> Result<O, E> {
        let endpoint_url = Self::get_url(base_url);
        let method = Self::METHOD;

        let request_builder = client.request(method.clone(), endpoint_url);

        let request_builder = match method {
            reqwest::Method::GET => request_builder.query(&self),
            _ => {
                let body = serde_json::to_string(&self)?;

                request_builder.body(body)
            }
        };

        let request_builder = if let Some(session) = session {
            request_builder.bearer_auth(session.token)
        } else {
            request_builder
        };

        let response = request_builder.send().await?;

        let response_body = response.text().await?;
        let result: O = serde_json::from_str(response_body.as_ref())?;

        Ok(result)
    }
}
