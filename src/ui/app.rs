use iced::Task;
use tokio::sync::mpsc;

use crate::{
    http,
    ws::{self},
};

use super::{Event, util::event_task};

pub struct Service {
    pub message_sender: Option<mpsc::UnboundedSender<ws::controller::SendEvent>>,
    http_client: http::api::Client,
}

impl Default for Service {
    fn default() -> Self {
        let http_client = http::api::Client::default();
        let message_sender = None;

        Self {
            message_sender,
            http_client,
        }
    }
}

impl Service {
    pub fn set_message_sender(&mut self, sender: mpsc::UnboundedSender<ws::controller::SendEvent>) {
        self.message_sender = Some(sender);
    }

    pub fn send_event(&self, event: ws::controller::SendEvent) -> iced::Task<Event> {
        if let Some(sender) = self.message_sender.clone() {
            if let Err(error) = sender.send(event) {
                tracing::error!("Send error {error}");

                event_task(Event::Error("Unable to connect to server".to_string()))
            } else {
                Task::none()
            }
        } else {
            event_task(Event::Error("Unable to connect to server".to_string()))
        }
    }

    pub fn get_messages(&self) {}
}
