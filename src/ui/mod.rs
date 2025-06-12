mod app;
mod parts;
mod theme;
mod util;
mod view;

use std::sync::Arc;

use iced::{Subscription, Task};
use parts::{chat, error_popup, login_form};
use tokio::sync::mpsc;
use util::event_task;

use crate::{auth, config, http, ws};

#[derive(Debug, Clone)]
enum AuthState {
    Authorized(auth::UserData),
    Unauthorized,
}

#[derive(Debug, Clone)]
pub enum Event {
    LoginForm(login_form::Event),
    Chat(chat::Event),
    ErrorPopup(error_popup::Event),
    FromWs(ws::controller::StreamItem),
    ToWs(ws::controller::SendEvent),
    Authenticated(auth::UserData),
}

pub struct Ui {
    theme: theme::App,
    state: State,
    chat: chat::Widget,
    login: login_form::Widget,
    error_popup: error_popup::Widget,
    ws_sender: Option<mpsc::UnboundedSender<ws::controller::SendEvent>>,
}

#[derive(Debug, Clone)]
struct State {
    auth_state: AuthState,
}

impl Default for Ui {
    fn default() -> Self {
        let theme = theme::Collection::default();
        let http_client = Arc::new(http::api::Client::default());

        let chat = chat::Widget {
            theme: theme.chat,
            http_client: http_client.clone(),
            state: chat::State::default(),
        };
        let login = login_form::Widget {
            theme: theme.login_form,
            http_client,
            state: login_form::State::default(),
        };
        let error = error_popup::Widget {
            theme: theme.error_popup,
            state: error_popup::State::default(),
        };

        let state = State {
            auth_state: AuthState::Unauthorized,
        };

        Self {
            theme: theme.app,
            state,
            chat,
            login,
            error_popup: error,
            ws_sender: None,
        }
    }
}

impl Ui {
    pub fn update(&mut self, event: Event) -> Task<Event> {
        match (self.state.auth_state.clone(), event) {
            (AuthState::Unauthorized, Event::LoginForm(event)) => self.login.update(event),
            (AuthState::Authorized(user_data), Event::Chat(event)) => {
                self.chat.update(user_data, event)
            }
            (_, Event::ErrorPopup(event)) => self.error_popup.update(event),
            (AuthState::Authorized(user_data), Event::FromWs(result)) => match result {
                Ok(event) => self.handle_controller_event(user_data, event),
                Err(error) => self.handle_controller_error(error),
            },
            (AuthState::Authorized(_), Event::ToWs(event)) => {
                if let Some(sender) = self.ws_sender.clone() {
                    if let Err(error) = sender.send(event) {
                        tracing::error!("Send error {error}");

                        event_task(Event::ErrorPopup(error_popup::Event::AddMessage(
                            "Unable to connect to server".to_string(),
                        )))
                    } else {
                        Task::none()
                    }
                } else {
                    event_task(Event::ErrorPopup(error_popup::Event::AddMessage(
                        "Unable to connect to server".to_string(),
                    )))
                }
            }
            (_, Event::Authenticated(user_data)) => {
                self.state.auth_state = AuthState::Authorized(user_data);

                let connect_event = chat::Event::Reconnect;
                event_task(Event::Chat(connect_event))
            }

            (AuthState::Authorized(_), Event::LoginForm(_)) => event_task(Event::ErrorPopup(
                error_popup::Event::AddMessage("Already authorized".to_string()),
            )),

            (AuthState::Unauthorized, _) => event_task(Event::ErrorPopup(
                error_popup::Event::AddMessage("Unauthorized".to_string()),
            )),
        }
    }

    fn handle_controller_error(&mut self, error: ws::controller::Error) -> Task<Event> {
        let message = match error {
            ws::controller::Error::Connection => "Connection error",
            ws::controller::Error::Send => "Send error",
            ws::controller::Error::Disconnected => "Disconnected",
            ws::controller::Error::Deserialization => "Deserialization error",
            ws::controller::Error::Serialization => "Serialization error",
            ws::controller::Error::Unknown => "Unknown error",
        };

        event_task(Event::ErrorPopup(error_popup::Event::AddMessage(
            message.to_string(),
        )))
    }

    fn handle_controller_event(
        &mut self,
        user_data: auth::UserData,
        event: ws::controller::Event,
    ) -> Task<Event> {
        match event {
            ws::controller::Event::Ready(ws_sender) => {
                self.ws_sender = Some(ws_sender);

                iced::Task::none()
            }
            ws::controller::Event::Message(message_content) => self
                .chat
                .update(user_data, chat::Event::Message(message_content)),

            ws::controller::Event::Disconnected => {
                self.chat.update(user_data, chat::Event::Disconnected)
            }

            ws::controller::Event::Connected => self.chat.update(user_data, chat::Event::Connected),
            ws::controller::Event::MessageSent => {
                self.chat.update(user_data, chat::Event::MessageSent)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Event> {
        Subscription::run(ws::controller::subscription).map(Event::FromWs)
    }
}
