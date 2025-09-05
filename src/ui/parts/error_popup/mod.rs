mod view;

use std::sync::Arc;

use iced::{Task, widget::scrollable};
use nultr_client_lib::{
    errors::IntoErrorMessage,
    ws::{self},
};
use nultr_shared_lib::request::{
    AuthenticatedUnexpectedErrorResponse, CreatePrivateRoomErrorResponse, GetMessagesErrorResponse,
    GetRoomsErrorResponse, GetUsersErrorResponse, LoginErrorResponse, UnexpectedErrorResponse,
};
use rust_api_kit::http::client::{RequestError, UnexpectedHttpError};

use crate::ui::{self, WidgetErrorEvent, theme};

#[derive(Debug, Clone)]
pub enum Event {
    AddError(ErrorEvent),
    DismissMessage(usize),
}

impl WidgetErrorEvent for Event {
    fn event(self) -> ui::Event {
        ui::Event::ErrorPopup(self)
    }

    fn task(self) -> Task<ui::Event> {
        self.event().task()
    }
}

#[derive(Debug, Clone)]
pub enum ErrorEvent {
    String(String),
    Unexpected(UnexpectedHttpError<UnexpectedErrorResponse>),
    AuthenticatedUnexpected(UnexpectedHttpError<AuthenticatedUnexpectedErrorResponse>),
    Login(LoginErrorResponse),
    GetMessages(GetMessagesErrorResponse),
    GetUsers(GetUsersErrorResponse),
    CreateRoom(CreatePrivateRoomErrorResponse),
    GetRooms(GetRoomsErrorResponse),
}

impl WidgetErrorEvent for ErrorEvent {
    fn event(self) -> ui::Event {
        ui::Event::ErrorPopup(Event::AddError(self))
    }

    fn task(self) -> Task<ui::Event> {
        self.event().task()
    }
}

type AppError = String;

#[derive(Debug, Clone)]
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
}

impl Widget {
    pub fn update(self: Arc<Self>, state: &mut State, event: Event) -> Task<ui::Event> {
        match event {
            Event::DismissMessage(index) => {
                state.error_messages.remove(index);

                Task::none()
            }
            Event::AddError(event) => self.error_update(state, event),
        }
    }

    pub fn error_update(self: Arc<Self>, state: &mut State, event: ErrorEvent) -> Task<ui::Event> {
        match event {
            ErrorEvent::String(message) => {
                state.error_messages.push(message);

                Task::none()
            }
            ErrorEvent::Unexpected(error) => {
                let message = match error {
                    UnexpectedHttpError::Request(request_error) => {
                        &self.get_request_error_message(request_error)
                    }
                    UnexpectedHttpError::Api(api_error) => match api_error {
                        UnexpectedErrorResponse::InternalServerError => "Server error",
                    },
                };

                ErrorEvent::String(message.to_string()).task()
            }
            ErrorEvent::AuthenticatedUnexpected(error) => {
                let message = match error {
                    UnexpectedHttpError::Request(request_error) => {
                        &self.get_request_error_message(request_error)
                    }
                    UnexpectedHttpError::Api(api_error) => match api_error {
                        AuthenticatedUnexpectedErrorResponse::InternalServerError => "Server error",
                        AuthenticatedUnexpectedErrorResponse::InvalidToken => "Invalid token",
                    },
                };

                ErrorEvent::String(message.to_string()).task()
            }
            ErrorEvent::Login(error) => ErrorEvent::String(error.into_error_message()).task(),
            ErrorEvent::GetUsers(_) => ErrorEvent::String("Unknown error".to_string()).task(),
            ErrorEvent::GetMessages(error) => ErrorEvent::String(error.into_error_message()).task(),
            ErrorEvent::GetRooms(error) => ErrorEvent::String(error.into_error_message()).task(),
            ErrorEvent::CreateRoom(_) => ErrorEvent::String("Unknown error".to_string()).task(),
        }
    }

    pub fn ws_update(self: Arc<Self>, error: ws::controller::Error) -> Task<ui::Event> {
        let message = match error {
            ws::controller::Error::Connection => "Connection error",
            ws::controller::Error::Send => "Send error",
            ws::controller::Error::Disconnected => "Disconnected",
            ws::controller::Error::Deserialization => "Deserialization error",
            ws::controller::Error::Serialization => "Serialization error",
            ws::controller::Error::Unknown => "Unknown error",
            ws::controller::Error::WrongRequestFormat => "Wrong request format",
            ws::controller::Error::UserNotFound => "User not found",
            ws::controller::Error::MessageNotFound(_) => "Unknown error",
            ws::controller::Error::NotMemberOfRoom => "User is not a member of room",
        };

        ErrorEvent::String(message.to_string()).task()
    }

    fn get_request_error_message(&self, error: RequestError) -> String {
        let message = match error {
            RequestError::Deserialize => "Deserialization error",
            RequestError::Builder => "Request builder error",
            RequestError::Http(status_code) => &format!("Http error: {status_code}"),
            RequestError::Timeout => "Request timeout",
            RequestError::Connect => "Connection error",
            RequestError::Redirect => "Redirect error",
            RequestError::Unknown => "Unknown error",
            RequestError::Decode => "Deserialization error",
        };

        message.to_string()
    }
}

impl From<UnexpectedHttpError<UnexpectedErrorResponse>> for ui::Event {
    fn from(value: UnexpectedHttpError<UnexpectedErrorResponse>) -> Self {
        ErrorEvent::Unexpected(value).event()
    }
}

impl From<UnexpectedHttpError<AuthenticatedUnexpectedErrorResponse>> for ui::Event {
    fn from(value: UnexpectedHttpError<AuthenticatedUnexpectedErrorResponse>) -> Self {
        ErrorEvent::AuthenticatedUnexpected(value).event()
    }
}
