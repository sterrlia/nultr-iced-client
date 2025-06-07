use std::fmt::Display;

use thiserror::Error;

use futures::{SinkExt, Stream, StreamExt as FuturesStreamExt, stream};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;
use url::Url;

type WsWriteStream = stream::SplitSink<
tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
tungstenite::Message,
>;
type WsReadStream = stream::SplitStream<
tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
>;

pub type ConnectionError = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Request {
    MessageToUser { user_id: i32, content: String }
}

#[derive(Debug, Clone)]
pub enum Response {
    Message(String),
}

#[derive(Error, Debug, Clone)]
pub enum RequestSendError {
    #[error("Send error: {0}")]
    Send(String),
    #[error("Disconnected")]
    Disconnected,
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Unknown error: {0}")]
    Error(String),
}

#[derive(Error, Debug, Clone)]
pub enum ResponseReceiveError {
    #[error("Unknown error: {0}")]
    Error(String),
    #[error("Serialization error: {0}")]
    Deserialization(String),
    #[error("Disconnected")]
    Disconnected,
}

pub enum State {
    Connected,
    Disconnected,
}

pub struct Instance {
    pub state: State,
    channel: Option<Channel>,
}

pub struct Channel {
    pub write_stream: WsWriteStream,
    pub read_stream: WsReadStream,
}

impl Channel {
    pub async fn send(&mut self, request: Request) -> Result<(), RequestSendError> {
        match request {
            Request::MessageToUser { user_id, content } => {
                let message = Request::MessageToUser {
                    user_id,
                    content
                };
                let serialized_message = serde_json::to_string(&message)
                    .map_err(|err| RequestSendError::Serialization(err.to_string()))?;

                let tungstenine_message = tungstenite::Message::Text(serialized_message.into());

                &mut self
                    .write_stream
                    .send(tungstenine_message)
                    .await
                    .map_err(|err| RequestSendError::Send(err.to_string()))?;

                Ok(())
            }
        }
    }

    pub async fn next(&mut self) -> Result<Response, ResponseReceiveError> {
        let ws_stream_value = &mut self.read_stream.next().await;
        let ws_event_result = match_ws_event(ws_stream_value);
        ws_event_result
    }
}

impl Instance {
    pub async fn connect(&mut self, url: Url) -> Result<(), ConnectionError> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .map_err(|err| err.to_string())?;

        let (ws_write, ws_read) = ws_stream.split();

        self.channel = Some(Channel {
            write_stream: ws_write,
            read_stream: ws_read,
        });

        self.state = State::Connected;

        Ok(())
    }

    pub async fn send(&mut self, request: Request) -> Result<(), RequestSendError> {
        if let Some(channel) = &mut self.channel {
            let result = channel.send(request).await;
            if result.is_err() {
                self.disconnect().await;
            }

            result
        } else {
            self.disconnect().await;
            Err(RequestSendError::Disconnected)
        }
    }

    pub async fn next(&mut self) -> Result<Response, ResponseReceiveError> {
        if let Some(channel) = &mut self.channel {
            channel.next().await
        } else {
            self.disconnect().await;
            Err(ResponseReceiveError::Disconnected)
        }
    }

    async fn disconnect(&mut self) {
        self.state = State::Disconnected;

        if let Some(channel) = &mut self.channel {
            channel
                .write_stream
                .close()
                .await
                .inspect_err(|err| warn!("Disconnect error {}", err.to_string()));

            self.channel = None;
        }
    }
}

fn match_ws_event(
    receive_event_value: &mut Option<Result<tungstenite::Message, tungstenite::Error>>,
) -> Result<Response, ResponseReceiveError> {
    #[derive(Debug, Deserialize)]
    enum WebsocketRequestEvent {
        UserMessage { content: String },
    }

    match receive_event_value {
        Some(Ok(receive_event)) => {
            let json = receive_event.to_string();
            let ws_message: Result<WebsocketRequestEvent, serde_json::Error> =
                serde_json::from_str(json.as_str());

            match ws_message {
                Ok(WebsocketRequestEvent::UserMessage { content }) => {
                    Ok(Response::Message(content))
                }
                Err(error) => {
                    let error_msg = error.to_string();
                    Err(ResponseReceiveError::Deserialization(error_msg))
                }
            }
        }
        Some(Err(error)) => Err(ResponseReceiveError::Error(error.to_string())),
        None => Err(ResponseReceiveError::Error("Channel closed".to_string())),
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            channel: None,
            state: State::Disconnected,
        }
    }
}
