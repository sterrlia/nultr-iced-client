mod render;
mod widgets;

use iced::Subscription;
use iced::widget::scrollable;

use crate::controller::{self, controller_subscription};
use crate::theme::{AppTheme, ChatTheme};

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    InputSubmitted,
    ShowMessageFromInput,
    Controller(controller::ReceiveEvent),
}

enum UserMessage {
    Incoming(String),
    Sent(String),
}

pub struct Ui {
    theme: ChatTheme,
    input_value: String,
    messages: Vec<UserMessage>,
    scroll: scrollable::Id,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            theme: AppTheme::default().chat,
            input_value: "".to_string(),
            messages: Vec::new(),
            scroll: scrollable::Id::new("1".to_string()),
        }
    }
}

impl Ui {
    pub fn update(&mut self, event: Event) -> () {
        match event {
            Event::InputChanged(new_value) => {
                self.input_value = new_value;
            }
            Event::InputSubmitted => {
                let input_value = self.get_input_value();

                if !input_value.is_empty() {}
            }
            Event::ShowMessageFromInput => {
                let message_content = self.get_input_value();
                self.messages.push(UserMessage::Sent(message_content));
                self.input_value.clear();
            }
            Event::Controller(event) => self.handle_controller_event(event),
        }
    }

    fn handle_controller_event(&mut self, event: controller::ReceiveEvent) -> () {
        match event {
            controller::ReceiveEvent::ConnectionError => {
            },
            controller::ReceiveEvent::ServerError => {
            },
            controller::ReceiveEvent::Error => {
            },
            controller::ReceiveEvent::Message(message_content) => {
                self.messages.push(UserMessage::Sent(message_content));
            }
            controller::ReceiveEvent::Disconnected => {
            }
            controller::ReceiveEvent::Connected => todo!(),
        }
    }

    fn get_input_value(&self) -> String {
        self.input_value.trim().to_string()
    }

    pub fn subscription(&self) -> Subscription<Event> {
        Subscription::run(controller_subscription).map(Event::Controller)
    }
}
