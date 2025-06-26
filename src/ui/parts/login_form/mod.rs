mod view;

use std::sync::Arc;

use iced::{Element, Task};
use rust_api_kit::http::client::{HttpClient, BasicHttpClientTrait};
use nultr_shared_lib::{
    request::{AuthUserData, LoginRequest, LoginResponse},
    util::MonoResult,
};

use crate::{ui::{self, theme, WidgetErrorEvent}, util::task_perform};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    UsernameChanged(String),
    PasswordChanged(String),
    InputSubmitted,
    LoginResult(LoginResponse),
}

impl Event {
    pub fn event(self) -> ui::Event {
        ui::Event::LoginForm(self)
    }

    pub fn task(self) -> Task<ui::Event> {
        self.event().task()
    }
}

#[derive(Debug, Clone)]
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
    pub http_client: Arc<HttpClient>,
}

impl Widget {
    pub fn update(self: Arc<Self>, state: &mut State, event: Event) -> Task<ui::Event> {
        match event {
            Event::UsernameChanged(username) => {
                state.username = username;

                Task::none()
            }
            Event::PasswordChanged(password) => {
                state.password = password;

                Task::none()
            }
            Event::InputSubmitted => task_perform(self.login(state.clone())),
            Event::LoginResult(response) => {
                let user_data = AuthUserData {
                    user_id: response.user_id,
                    token: response.token,
                };

                ui::Event::Authenticated(user_data).task()
            }
        }
    }

    async fn login(self: Arc<Self>, state: State) -> MonoResult<ui::Event> {
        let request = LoginRequest {
            username: state.username.clone(),
            password: state.password.clone(),
        };

        let result = self.http_client.request(request).await?;
        Ok(match result {
            Ok(response) => Event::LoginResult(response).event(),
            Err(error) => error_popup::ErrorEvent::Login(error).event(),
        })
    }
}
