use futures::{SinkExt, StreamExt as FuturesStreamExt};
use tokio_stream::StreamExt;
use iced::futures::{stream, Stream};
use tokio::{select, sync::mpsc};
use tokio_tungstenite::tungstenite;

use crate::config;

enum Event {
    Send(SendEvent),
    Receive(ReceiveEvent)
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

pub fn connect_to_chat() -> impl Stream<Item = ReceiveEvent> {
    stream::unfold(100, |mut output| async move {
        let url = config::VARIABLES.ws_url.clone();

        // Connect to the WebSocket server
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");
        println!("WebSocket connection established");

        let (mut write, mut read) = ws_stream.split();

        let (send_tx, mut send_rx) = mpsc::unbounded_channel::<SendEvent>();

        while let Some(event) = select! {
            send_event_value = send_rx.recv() => {
                if let Some(send_event) = send_event_value {
                    Some(Event::Send(send_event))
                } else {
                    None
                }
            },
            receive_event_value = read.next() => {
                if let Some(receive_event) = receive_event_value {
                    Some(Event::Receive(receive_event))
                } else {
                    None
                }
            }
        } {
            handle_event(event);
        }

        // Optionally close the connection
        write.send(tungstenite::Message::Close(None)).await.expect("Failed to close connection");

        todo!()
    })
}

fn handle_event(event: Event, ) {
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

