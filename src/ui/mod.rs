mod parts;
mod theme;
mod view;

use std::sync::Arc;

use nultr_client_lib::{config, ws};
use iced::{Subscription, Task};
use parts::{chat, error_popup, login_form};
use rust_api_kit::http::client::{BearerToken, HttpClient, HttpClientTrait};
use nultr_shared_lib::request::AuthUserData;

#[derive(Debug, Clone)]
pub enum Event {
    LoginForm(login_form::Event),
    Chat(chat::Event),
    ErrorPopup(error_popup::Event),
    FromWs(Result<ws::controller::Event, ws::controller::Error>),
    Authenticated(AuthUserData),
}

trait WidgetErrorEvent {
    fn event(self) -> Event;
    fn task(self) -> Task<Event>;
}

impl Event {
    pub fn task(self) -> Task<Event> {
        Task::perform(async { self }, |value| value)
    }
}

pub struct Ui {
    theme: theme::App,
    chat: Arc<chat::Widget>,
    login: Arc<login_form::Widget>,
    error_popup: Arc<error_popup::Widget>,
    state: State,
    auth_state: AuthState,
}

struct State {
    chat: chat::State,
    login_form: login_form::State,
    error_popup: error_popup::State,
}

impl Default for State {
    fn default() -> Self {
        let chat = chat::State::default();
        let login_form = login_form::State::default();
        let error_popup = error_popup::State::default();

        Self {
            chat,
            login_form,
            error_popup,
        }
    }
}

#[derive(Clone, Debug)]
enum AuthState {
    Authenticated(AuthUserData),
    Unauthenticated,
}

impl Default for Ui {
    fn default() -> Self {
        let theme = theme::Collection::default();
        let base_url = config::get_variables().http_url.clone();
        let http_client = Arc::new(HttpClient::new(base_url));

        let chat = Arc::new(chat::Widget {
            theme: theme.chat,
            http_client: http_client.clone(),
        });
        let login = Arc::new(login_form::Widget {
            theme: theme.login_form,
            http_client,
        });
        let error = Arc::new(error_popup::Widget {
            theme: theme.error_popup,
        });

        let auth_state = AuthState::Unauthenticated;

        let state = State::default();

        Self {
            auth_state,
            theme: theme.app,
            chat,
            login,
            state,
            error_popup: error,
        }
    }
}

impl Ui {
    pub fn update(&mut self, event: Event) -> Task<Event> {
        match (self.auth_state.clone(), event) {
            (_, Event::ErrorPopup(event)) => {
                self.error_popup.clone().update(&mut self.state.error_popup, event)
            }

            (_, Event::FromWs(Ok(ws::controller::Event::Ready(sender)))) => {
                self.state.chat.ws_sender = sender;

                Task::none()
            }

            (AuthState::Unauthenticated, Event::LoginForm(event)) => {
                self.login.clone().update(&mut self.state.login_form, event)
            }
            (AuthState::Authenticated(user_data), Event::Chat(event)) => {
                self.chat.clone().update(&mut self.state.chat, user_data, event)
            }

            (AuthState::Authenticated(user_data), Event::FromWs(result)) => match result {
                Ok(event) => self.chat.clone().ws_update(&mut self.state.chat, event),
                Err(error) => self.error_popup.clone().ws_update(error),
            },
            (_, Event::Authenticated(user_data)) => {
                self.auth_state = AuthState::Authenticated(user_data);

                chat::Event::Reconnect.task()
            }

            (AuthState::Authenticated(_), Event::LoginForm(_)) => {
                error_popup::ErrorEvent::String("Already authorized".to_string()).task()
            }

            (AuthState::Unauthenticated, event) => {
                tracing::error!("{:?}", event);

                error_popup::ErrorEvent::String("Unauthorized".to_string()).task()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Event> {
        Subscription::run(ws::controller::iced_integration::subscription).map(Event::FromWs)
    }
}
