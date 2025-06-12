mod view;

use std::sync::Arc;

use iced::{Element, Task};

use crate::{
    auth,
    http::{
        self,
        models::{ApiError, LoginRequest, LoginResponse},
    },
    ui::{self, app, theme, util::event_task},
};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    UsernameChanged(String),
    PasswordChanged(String),
    InputSubmitted,
    LoginResult(LoginResponse),
}

pub struct State {
    username: String,
    password: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }
}

pub struct Widget {
    pub theme: theme::LoginForm,
    pub state: State,
    pub http_client: Arc<http::api::Client>,
}

impl Widget {
    pub fn update(&mut self, event: Event) -> Task<ui::Event> {
        match event {
            Event::UsernameChanged(username) => {
                self.state.username = username;

                Task::none()
            }
            Event::PasswordChanged(password) => {
                self.state.password = password;

                Task::none()
            }
            Event::InputSubmitted => {
                let request = LoginRequest {
                    username: self.state.username.clone(),
                    password: self.state.password.clone(),
                };

                let http_client = self.http_client.clone();

                Task::perform(
                    async move { http_client.clone().request(request, None).await },
                    |result| match result {
                        Ok(response) => ui::Event::LoginForm(Event::LoginResult(response)),
                        Err(error) => ui::Event::ErrorPopup(error_popup::Event::AddApiError(error)),
                    },
                )
            }
            Event::LoginResult(response) => {
                let user_data = auth::UserData {
                    user_id: response.user_id,
                    token: response.token,
                };

                event_task(ui::Event::Authenticated(user_data))
            }
        }
    }
}
