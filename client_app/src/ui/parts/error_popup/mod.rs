mod view;

use std::sync::Arc;

use iced::{Element, Task, widget::scrollable};

use crate::{
    http,
    ui::{self, theme, util::event_task},
};

#[derive(Debug, Clone)]
pub enum Event {
    AddMessage(String),
    AddApiError(http::api::Error<http::models::ErrorResponse>),
    DismissMessage(usize),
}

type AppError = String;

pub struct State {
    error_messages_scrollable: scrollable::Id,
    error_messages: Vec<AppError>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            error_messages: Vec::new(),
            error_messages_scrollable: scrollable::Id::new("2"),
        }
    }
}

pub struct Widget {
    pub theme: theme::ErrorPopup,
    pub state: State,
}

impl Widget {
    pub fn update(&mut self, event: Event) -> Task<ui::Event> {
        match event {
            Event::DismissMessage(index) => {
                self.state.error_messages.remove(index);

                iced::Task::none()
            }
            Event::AddMessage(message) => {
                self.state.error_messages.push(message);

                iced::Task::none()
            }

            Event::AddApiError(error) => {
                let message = match error {
                    http::api::Error::Request(request_error) => match request_error {
                        http::api::RequestError::Builder => "Request builder error",
                        http::api::RequestError::Http(_) => "Request error",
                        http::api::RequestError::Timeout => "Http timeout",
                        http::api::RequestError::Connect => "Connection error",
                        http::api::RequestError::Redirect => "Redirect error",
                        http::api::RequestError::Unknown => "Unknown request error",
                        http::api::RequestError::Decode => "Error while decoding",
                        http::api::RequestError::Deserialize => "Deserialization error",
                    },

                    http::api::Error::Api(error_response) => match error_response {
                        http::models::ErrorResponse::InternalServerError => "Unknown api error",
                        http::models::ErrorResponse::UserNotFound => "User not found",
                        http::models::ErrorResponse::AccessDenied => "Access denied",
                        http::models::ErrorResponse::InvalidToken => "Invalid jwt token",
                    },
                };

                event_task(ui::Event::ErrorPopup(Event::AddMessage(
                    message.to_string(),
                )))
            }
        }
    }
}
