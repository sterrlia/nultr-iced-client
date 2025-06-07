mod app;
mod render;
mod util;
mod widgets;

use iced::Subscription;
use iced::widget::scrollable;
use util::event_task;

use crate::theme::{AppTheme, ChatTheme};
use crate::{config, ws};

enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    SendMessage,
    Controller(ws::controller::StreamItem),
    DismissError(usize),
    Connect,
    Error(String),
}

enum UserMessage {
    Received(String),
    Sent(String),
}

type AppError = String;

pub struct Ui {
    theme: ChatTheme,
    state: State,
    service: app::Service,
}

struct UserData {
    user_id: i32,
    token: String,
}

struct State {
    input_value: String,
    messages: Vec<UserMessage>,
    error_messages: Vec<AppError>,
    messages_scrollable: scrollable::Id,
    error_messages_scrollable: scrollable::Id,
    connection_state: ConnectionState,
    selected_user_id: Option<i32>,
    logged_user_data: Option<UserData>,
}

impl Default for Ui {
    fn default() -> Self {
        let theme = AppTheme::default().chat;
        let service = app::Service::default();

        let state = State {
            input_value: "".to_string(),
            messages: Vec::new(),
            error_messages: Vec::new(),
            messages_scrollable: scrollable::Id::new("1"),
            error_messages_scrollable: scrollable::Id::new("2"),
            connection_state: ConnectionState::Disconnected,
            selected_user_id: None,
            logged_user_data: None,
        };

        Self {
            theme,
            state,
            service,
        }
    }
}

impl Ui {
    pub fn update(&mut self, event: Event) -> iced::Task<Event> {
        match event {
            Event::InputChanged(new_value) => {
                self.state.input_value = new_value;

                iced::Task::none()
            }
            Event::SendMessage => {
                let input_value = self.state.input_value.trim().to_string();

                if !input_value.is_empty() {
                    return iced::Task::none();
                };

                if let Some(user_id) = self.state.selected_user_id {
                    self.state
                        .messages
                        .push(UserMessage::Sent(input_value.clone()));

                    self.state.input_value.clear();

                    let controller_event = ws::controller::SendEvent::Message {
                        user_id,
                        content: input_value,
                    };

                    self.service.send_event(controller_event)
                } else {
                    event_task(Event::Error("User is not chosen".to_string()))
                }
            }
            Event::Controller(result) => match result {
                Ok(event) => self.handle_controller_event(event),
                Err(error) => self.handle_controller_error(error),
            },
            Event::DismissError(index) => {
                self.state.error_messages.remove(index);

                iced::Task::none()
            }
            Event::Connect => {
                let ws_url = config::get_variables().ws_url.clone();
                let controller_event = ws::controller::SendEvent::Connect(ws_url);

                self.service.send_event(controller_event)
            }
            Event::Error(message) => {
                self.state.error_messages.push(message);

                iced::Task::none()
            }
        }
    }

    fn handle_controller_error(&mut self, error: ws::controller::Error) -> iced::Task<Event> {
        let message = match error {
            ws::controller::Error::Connection => "Connection error",
            ws::controller::Error::Send => "Send error",
            ws::controller::Error::Disconnected => "Disconnected",
            ws::controller::Error::Deserialization => "Deserialization error",
            ws::controller::Error::Serialization => "Serialization error",
            ws::controller::Error::Unknown => "Unknown error",
        };

        event_task(Event::Error(message.to_string()))
    }

    fn handle_controller_event(&mut self, event: ws::controller::Event) -> iced::Task<Event> {
        match event {
            ws::controller::Event::Ready(message_sender) => {
                self.service.set_message_sender(message_sender);
            }
            ws::controller::Event::Message(message_content) => {
                self.state
                    .messages
                    .push(UserMessage::Received(message_content));
            }
            ws::controller::Event::Disconnected => {
                self.state.connection_state = ConnectionState::Disconnected;
            }
            ws::controller::Event::Connected => {
                self.state.connection_state = ConnectionState::Connected;
            }
            ws::controller::Event::MessageSent => {}
        };

        iced::Task::none()
    }

    pub fn subscription(&self) -> Subscription<Event> {
        Subscription::run(ws::controller::subscription).map(Event::Controller)
    }
}
