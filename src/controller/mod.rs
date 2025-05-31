pub mod websocket;

use async_stream::stream;
use futures::{SinkExt, Stream, StreamExt as FuturesStreamExt, stream};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::config;

type ReceivedSendEventResult = Option<SendEvent>;
type ReceivedReceiveEventResult = Result<websocket::ResponseEvent, websocket::StreamError>;

enum Event {
    Send(ReceivedSendEventResult),
    Receive(ReceivedReceiveEventResult),
}

#[derive(Debug, Clone)]
pub enum ReceiveEvent {
    Connected,
    Message(String),
    Disconnected,
    ConnectionError,
    ServerError,
    Error,
}

#[derive(Debug, Clone)]
pub enum SendEvent {
    Connect,
    Message(String),
}

pub fn controller_subscription() -> impl Stream<Item = ReceiveEvent> {
    stream! {
        let mut handler = EventHandler::default();

        loop {
            let event = handler.next().await;

            let event_result = match event {
                Event::Send(result) => {
                    handler.handle_send(result).await
                },
                Event::Receive(result) => {
                    Some(handler.handle_receive(result).await)
                }
            };

            if let Some(return_event) = event_result {
                yield return_event;
            }
        }
    }
}

struct EventHandler {
    ws_client: websocket::Client,
    send_tx: UnboundedSender<SendEvent>,
    send_rx: UnboundedReceiver<SendEvent>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (send_tx, mut send_rx) = mpsc::unbounded_channel::<SendEvent>();

        Self {
            ws_client: websocket::Client::default(),
            send_tx,
            send_rx,
        }
    }
}

impl EventHandler {
    pub async fn next(&mut self) -> Event {
        match self.ws_client.state {
            websocket::ClientState::Connected => select! {
                send_event_value = self.send_rx.recv() => {
                    Event::Send(send_event_value)
                },
                receive_event_value = self.ws_client.next() => {
                    Event::Receive(receive_event_value)
                }
            },
            websocket::ClientState::Disconnected => {
                let event = self.send_rx.recv().await;
                Event::Send(event)
            }
        }
    }

    pub async fn handle_send(&mut self, event_result: ReceivedSendEventResult) -> Option<ReceiveEvent> {
        if let Some(send_event) = event_result {
            match send_event {
                SendEvent::Connect => {
                    let url = config::get_variables().ws_url.clone();
                    let result = self.ws_client.connect(url).await;

                    match result {
                        Err(_) => Some(ReceiveEvent::ConnectionError),
                        Ok(_) => None
                    }
                },
                SendEvent::Message(content) => {
                    let request = websocket::Request::Message(content);
                    let result = self.ws_client.send(request).await;

                    match result {
                        Err(error) => Some(error.into()),
                        Ok(_) => None
                    }
                }
            }
        } else {
            Some(ReceiveEvent::Error)
        }
    }
    pub async fn handle_receive(
        &mut self,
        event_result: ReceivedReceiveEventResult,
    ) -> ReceiveEvent {
        match event_result {
            Ok(response_event) => response_event.into(),
            Err(response_event_error) => response_event_error.into(),
        }
    }
}

impl From<websocket::StreamError> for ReceiveEvent {
    fn from(value: websocket::StreamError) -> Self {
        match value {
            websocket::StreamError::Fatal => ReceiveEvent::Error,
            websocket::StreamError::Deserialization(_) => ReceiveEvent::ServerError,
            websocket::StreamError::Disconnected => ReceiveEvent::Disconnected,
        }
    }
}

impl From<websocket::RequestSendError> for ReceiveEvent {
    fn from(value: websocket::RequestSendError) -> Self {
        match value {
            websocket::RequestSendError::Disconnected => ReceiveEvent::Disconnected,
            websocket::RequestSendError::Fatal => ReceiveEvent::Error
        }
    }
}

impl From<websocket::ResponseEvent> for ReceiveEvent {
    fn from(value: websocket::ResponseEvent) -> Self {
        match value {
            websocket::ResponseEvent::Message(content) => ReceiveEvent::Message(content),
        }
    }
}
