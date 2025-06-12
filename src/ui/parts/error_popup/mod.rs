mod view;

use std::sync::Arc;

use iced::{widget::scrollable, Element, Task};

use crate::{http::models::ApiError, ui::{self, app, theme, util::event_task}};

#[derive(Debug, Clone)]
pub enum Event {
    AddMessage(String),
    AddApiError(ApiError),
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
                    ApiError::Deserialization => "Deserialization error",
                    ApiError::Http(_) => "Request error",
                    ApiError::Timeout => "Http timeout",
                    ApiError::Connect => "Connection error",
                    ApiError::Redirect => "Redirect error",
                    ApiError::Unknown => "Unknown api error",
                    ApiError::Decode => "Error while decoding"
                };

                event_task(ui::Event::ErrorPopup(Event::AddMessage(message.to_string())))
            }
        }
    }
}


