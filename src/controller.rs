use futures::{channel::mpsc::UnboundedSender, SinkExt, StreamExt as FuturesStreamExt};
use iced::futures::{stream, Stream};
use serde::{Deserialize, Serialize};
use tokio::{select, sync::mpsc};
use tokio_tungstenite::tungstenite::{self, Message};
use url::Url;

use crate::{config, logger};

enum Event {
    Send(Result<SendEvent, SendEventError>),
    Receive(Result<ReceiveEvent, ReceiveEventError>)
}

#[derive(Debug, Deserialize)]
enum WebsocketReceiveEvent {
    UserMessage {
        content: String
    }
}

#[derive(Debug, Clone)]
pub enum ReceiveEvent {
    Connected,
    Message(String),
    Disconnected
}

#[derive(Debug, Clone)]
pub enum SendEvent {
    Message(String)
}

enum ReceiveEventError {
    ReceiveError(String),
    Deserialization(String),
    NoneReceived,
}

enum SendEventError {
    NoneReceived
}



type WsWriteStream = stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WsReadStream = stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;
type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WsChannelError = String;

struct WsChannel {
    pub write_stream: WsWriteStream,
    pub read_stream: WsReadStream
}

impl WsChannel {
    async fn new(url: Url) -> Result<WsChannel, WsChannelError> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .map_err(|err| err.to_string())?;

        let (ws_write, ws_read) = ws_stream.split();

        Ok(WsChannel {
            write_stream: ws_write,
            read_stream: ws_read
        })
    }
}

impl Drop for WsChannel {
    fn drop(&mut self) {
        self.write_stream.close();
    }
}

pub fn connect_to_chat() -> impl Stream<Item = ReceiveEvent> {
    stream::unfold(100, |mut output| async move {
        let url = config::VARIABLES.ws_url.clone();

        let ws_channel = WsChannel::new(url).await;

        let (send_tx, mut send_rx) = mpsc::unbounded_channel::<SendEvent>();

        loop {
            let event = select! {
                send_event_value = send_rx.recv() => match_send_event(send_event_value),
                receive_event_value = ws_channel.read_stream.next() => match_receive_event(receive_event_value)
            };

            handle_event(event, send_tx.clone());
        }

        // Optionally close the connection
        ws_write.send(tungstenite::Message::Close(None)).await.expect("Failed to close connection");

        todo!()
    })
}

fn match_receive_event(
    receive_event_value: Option<Result<tungstenite::Message, tungstenite::Error>>
) -> Event {
    match receive_event_value {
        Some(Ok(receive_event)) => {
            let json = receive_event.to_string();
            let ws_message: Result<WebsocketReceiveEvent, serde_json::Error> = serde_json::from_str(json.as_str());

            match ws_message {
                Ok(WebsocketReceiveEvent::UserMessage { content }) => {
                    Event::Receive(Ok(ReceiveEvent::Message(content)))
                }
                Err(error) => {
                    let error_msg = error.to_string();
                    Event::Receive(Err(ReceiveEventError::Deserialization(error_msg)))
                }
            }
        },
        Some(Err(error)) => {
            let error_msg = error.to_string();
            Event::Receive(Err(ReceiveEventError::ReceiveError(error_msg)))
        },
        None => Event::Receive(Err(ReceiveEventError::NoneReceived))
    }
}

fn match_send_event(send_event_value: Option<SendEvent>) -> Event {
    if let Some(send_event) = send_event_value {
        Event::Send(Ok(send_event))
    } else {
        Event::Send(Err(SendEventError::NoneReceived))
    }
}

fn handle_event(
    event: Event, 
    sender: mpsc::UnboundedSender<SendEvent>,
) {
    match event {
        Event::Send(send_event) => {
            match send_event {
                SendEvent::Message(content) => {
                    let msg = tungstenite::Message::Text(content.into());
                    write.send(msg).await.expect("Failed to send message");
                }
            }
        }
        Event::Receive(receive_event) => {
        }
    };
}

