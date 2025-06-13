use reqwest::StatusCode;
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
    pub async fn request<I, O, E>(
        &self,
        request: I,
        session: Option<Session>,
    ) -> Result<O, Error<E>>
    where
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
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
    E: for<'de> Deserialize<'de>,
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
    ) -> Result<O, Error<E>> {
        let endpoint_url = Self::get_url(base_url);
        let method = Self::METHOD;

        let request_builder = client.request(method.clone(), endpoint_url);

        let request_builder = match method {
            reqwest::Method::GET => request_builder.query(&self),
            _ => request_builder.json(&self),
        };

        let request_builder = if let Some(session) = session {
            request_builder.bearer_auth(session.token)
        } else {
            request_builder
        };

        let response = request_builder
            .send()
            .await
            .map_err(|err| Error::<E>::from(err))?;

        let status = response.status();

        if status.is_success() {
            let body_raw = response.text().await.map_err(|err| Error::<E>::from(err))?;
            println!("body raw {}", body_raw);

            let body: O = serde_json::from_str(body_raw.as_str())
                .map_err(|err| Error::<E>::from((err, body_raw)))?;

            Ok(body)
        } else {
            let body_raw = response.text().await.map_err(|err| Error::<E>::from(err))?;

            let body: E = serde_json::from_str(body_raw.as_str())
                .map_err(|err| Error::<E>::from((err, body_raw)))?;

            Err(Error::Api(body))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Error<T>
where
    T: for<'de> Deserialize<'de>,
{
    Request(RequestError),
    Api(T),
}

#[derive(Clone, Debug)]
pub enum RequestError {
    Deserialize,
    Builder,
    Http(StatusCode),
    Timeout,
    Connect,
    Redirect,
    Unknown,
    Decode,
}

impl<T> From<(serde_json::Error, String)> for Error<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: (serde_json::Error, String)) -> Self {
        let (error, body) = value;
        tracing::error!("Deserialization error {:?}, body was: '{}'", error, body);

        Error::<T>::Request(RequestError::Deserialize)
    }
}

impl<T> From<reqwest::Error> for Error<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: reqwest::Error) -> Self {
        tracing::error!("Request error {:?}", value);

        let request_error = if let Some(status) = value.status() {
            RequestError::Http(status)
        } else if value.is_timeout() {
            RequestError::Timeout
        } else if value.is_connect() {
            RequestError::Connect
        } else if value.is_redirect() {
            RequestError::Redirect
        } else if value.is_decode() {
            RequestError::Decode
        } else if value.is_builder() {
            RequestError::Builder
        } else {
            RequestError::Unknown
        };

        Error::<T>::Request(request_error)
    }
}
