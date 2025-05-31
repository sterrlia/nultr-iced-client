use log::{info, trace, warn};
use futures::{SinkExt, Stream, StreamExt as FuturesStreamExt, stream};
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

pub type WsChannelConnectionError = String;

#[derive(Debug, Clone)]
pub enum ResponseEvent {
    Message(String),
}

#[derive(Debug, Clone)]
pub enum Request {
    Message(String),
}

#[derive(Debug, Clone)]
pub enum RequestSendError {
    Disconnected,
    Fatal,
}

#[derive(Debug, Clone)]
pub enum StreamError {
    Fatal,
    Deserialization(String),
    Disconnected,
}

pub enum ClientState {
    Connected,
    Disconnected
}

pub struct Client {
    pub state: ClientState,
    channel: Option<Channel>,
}

pub struct Channel {
    pub write_stream: WsWriteStream,
    pub read_stream: WsReadStream,
}

impl Channel {
    pub async fn send(&mut self, request: Request) -> Result<(), RequestSendError> {
        match request {
            Request::Message(content) => {
                let msg = tungstenite::Message::Text(content.into());

                &mut self.write_stream.send(msg).await.map_err(|err| {
                    let error: RequestSendError = err.into();
                    error
                })?;

                Ok(())
            }
        }
    }

    pub async fn next(&mut self) -> Result<ResponseEvent, StreamError> {
        let ws_stream_value = &mut self.read_stream.next().await;
        let ws_event_result = match_ws_event(ws_stream_value);
        ws_event_result
    }
}

impl Client {
    pub async fn connect(&mut self, url: Url) -> Result<(), WsChannelConnectionError> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .map_err(|err| err.to_string())?;

        let (ws_write, ws_read) = ws_stream.split();

        self.channel = Some(Channel {
            write_stream: ws_write,
            read_stream: ws_read,
        });

        self.state = ClientState::Connected;

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

    pub async fn next(&mut self) -> Result<ResponseEvent, StreamError> {
        if let Some(channel) = &mut self.channel {
            channel.next().await
        } else {
            self.disconnect().await;
            Err(StreamError::Disconnected)
        }
    }

    async fn disconnect(&mut self) {
        self.state = ClientState::Disconnected;

        if let Some(channel) = &mut self.channel {
            channel.write_stream.close()
                .await
                .inspect_err(|err| warn!("Disconnect error {}", err.to_string()));

            self.channel = None;
        }
    }
}

fn match_ws_event(
    receive_event_value: &mut Option<Result<tungstenite::Message, tungstenite::Error>>,
) -> Result<ResponseEvent, StreamError> {
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
                    Ok(ResponseEvent::Message(content))
                }
                Err(error) => {
                    let error_msg = error.to_string();
                    Err(StreamError::Deserialization(error_msg))
                }
            }
        }
        Some(Err(error)) => Err(error.into()),
        None => Err(StreamError::Fatal),
    }
}

impl Default for Client {
    fn default() -> Self {
        Self { 
            channel: None,
            state: ClientState::Disconnected
        }
    }
}

impl From<tungstenite::Error> for RequestSendError {
    fn from(value: tungstenite::Error) -> Self {
        match value {
            tungstenite::Error::AlreadyClosed | tungstenite::Error::ConnectionClosed => {
                RequestSendError::Disconnected
            }
            _ => RequestSendError::Fatal,
        }
    }
}

impl From<&mut tungstenite::Error> for StreamError {
    fn from(value: &mut tungstenite::Error) -> Self {
        match value {
            tungstenite::Error::AlreadyClosed | tungstenite::Error::ConnectionClosed => {
                StreamError::Disconnected
            }
            _ => StreamError::Fatal,
        }
    }
}
