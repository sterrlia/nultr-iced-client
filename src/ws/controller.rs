use async_stream::stream;
use futures::{SinkExt, Stream, StreamExt as FuturesStreamExt, stream};
use tokio::{
    select,
    sync::mpsc::{self},
};
use url::Url;

use super::client;

type ReceivedReceiveEventResult = Result<client::Response, client::ResponseReceiveError>;
pub type StreamItem = Result<Event, Error>;

enum ReceivedEventVariant {
    Send(SendEvent),
    Receive(ReceivedReceiveEventResult),
}

#[derive(Debug, Clone)]
pub enum Event {
    Ready(mpsc::UnboundedSender<SendEvent>),
    Connected,
    Message(String),
    MessageSent,
    Disconnected,
}

#[derive(Debug, Clone)]
pub enum Error {
    Send,
    Connection,
    Disconnected,
    Deserialization,
    Serialization,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum SendEvent {
    Connect(Url),
    Message { user_id: i32, content: String },
}

pub fn subscription() -> impl Stream<Item = StreamItem> {
    stream! {
        let (send_tx, send_rx) = mpsc::unbounded_channel::<SendEvent>();

        let mut handler = EventHandler::new(send_rx);

        yield Ok(Event::Ready(send_tx));

        loop {
            let event = handler.next().await;

            let event_result = match event {
                ReceivedEventVariant::Send(result) => {
                    handler.handle_send(result).await
                },
                ReceivedEventVariant::Receive(result) => {
                    handler.handle_receive(result).await
                }
            };

            yield event_result;
        }
    }
}

struct EventHandler {
    ws_client: client::Instance,
    send_rx: mpsc::UnboundedReceiver<SendEvent>,
}

impl EventHandler {
    pub fn new(send_rx: mpsc::UnboundedReceiver<SendEvent>) -> Self {
        Self {
            ws_client: client::Instance::default(),
            send_rx,
        }
    }

    pub async fn next(&mut self) -> ReceivedEventVariant {
        match self.ws_client.state {
            client::State::Connected => select! {
                send_event_value = self.send_rx.recv() => {
                    ReceivedEventVariant::Send(send_event_value.unwrap())
                },
                receive_event_value = self.ws_client.next() => {
                    ReceivedEventVariant::Receive(receive_event_value)
                }
            },
            client::State::Disconnected => {
                let event = self.send_rx.recv().await;
                ReceivedEventVariant::Send(event.unwrap())
            }
        }
    }

    pub async fn handle_send(&mut self, event: SendEvent) -> StreamItem {
        match event {
            SendEvent::Connect(url) => {
                let result = self.ws_client.connect(url).await;

                match result {
                    Err(error) => Err(error.into()),
                    Ok(_) => Ok(Event::Connected),
                }
            }
            SendEvent::Message { user_id, content } => {
                let request = client::Request::MessageToUser { user_id, content };
                let result = self.ws_client.send(request).await;

                match result {
                    Err(error) => Err(error.into()),
                    Ok(_) => Ok(Event::MessageSent),
                }
            }
        }
    }
    pub async fn handle_receive(&mut self, event_result: ReceivedReceiveEventResult) -> StreamItem {
        match event_result {
            Ok(response_event) => match response_event {
                client::Response::Message(content) => Ok(Event::Message(content)),
            },
            Err(response_event_error) => Err(response_event_error.into()),
        }
    }
}

impl From<client::ResponseReceiveError> for Error {
    fn from(value: client::ResponseReceiveError) -> Self {
        log::error!("{}", value.to_string());

        match value {
            client::ResponseReceiveError::Error(_) => Error::Unknown,
            client::ResponseReceiveError::Deserialization(_) => Error::Deserialization,
            client::ResponseReceiveError::Disconnected => Error::Disconnected,
        }
    }
}

impl From<client::RequestSendError> for Error {
    fn from(value: client::RequestSendError) -> Self {
        log::error!("{}", value.to_string());

        match value {
            client::RequestSendError::Send(_) => Error::Send,
            client::RequestSendError::Disconnected => Error::Disconnected,
            client::RequestSendError::Serialization(_) => Error::Serialization,
            client::RequestSendError::Error(_) => Error::Unknown,
        }
    }
}

impl From<client::ConnectionError> for Error {
    fn from(value: client::ConnectionError) -> Self {
        log::error!("{}", value.to_string());

        Error::Connection
    }
}
