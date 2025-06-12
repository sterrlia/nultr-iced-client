mod view;

use std::sync::Arc;

use iced::{Element, Task, widget::scrollable};

use crate::{
    auth, config,
    http::{
        self,
        models::{GetMessagesRequest, GetUsersRequest, MessageResponse, Pagination, UserResponse},
    },
    ui::{self, app, theme, util::event_task},
    ws,
};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    SendMessage,
    Message(String),
    MessageSent,
    Reconnect,
    Connected,
    Disconnected,
    LoadMessages,
    LoadUsers,
    AddUsers(Vec<UserResponse>),
    AddMessages(Vec<MessageResponse>),
}

struct User {
    pub id: i32,
    pub username: String,
}

pub enum ConnectionState {
    Connected,
    Disconnected,
}

pub struct State {
    input_value: String,
    users: Vec<User>,
    messages: Vec<UserMessage>,
    messages_scrollable: scrollable::Id,
    selected_user_id: Option<i32>,
    connection_state: ConnectionState,
    next_messages_page: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_value: "".to_string(),
            users: Vec::new(),
            messages: Vec::new(),
            selected_user_id: None,
            messages_scrollable: scrollable::Id::new("1"),
            connection_state: ConnectionState::Disconnected,
            next_messages_page: 1,
        }
    }
}

enum UserMessage {
    Received(String),
    Sent(String),
}

pub struct Widget {
    pub theme: theme::ChatTheme,
    pub state: State,
    pub http_client: Arc<http::api::Client>,
}

impl Widget {
    pub fn update(&mut self, logged_user_data: auth::UserData, event: Event) -> Task<ui::Event> {
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

                    event_task(ui::Event::ToWs(controller_event))
                } else {
                    event_task(ui::Event::ErrorPopup(error_popup::Event::AddMessage(
                        "User is not chosen".to_string(),
                    )))
                }
            }
            Event::Message(content) => {
                self.state.messages.push(UserMessage::Received(content));

                iced::Task::none()
            }
            Event::MessageSent => iced::Task::none(),
            Event::Reconnect => {
                let ws_url = config::get_variables().ws_url.clone();

                let disconnect_event = ws::controller::SendEvent::Disconnect;
                let connect_event = ws::controller::SendEvent::Connect {
                    url: ws_url,
                    token: logged_user_data.token,
                };

                event_task(ui::Event::ToWs(disconnect_event))
                    .chain(event_task(ui::Event::ToWs(connect_event)))
            }
            Event::Connected => {
                self.state.connection_state = ConnectionState::Connected;

                iced::Task::none()
            }
            Event::Disconnected => {
                self.state.connection_state = ConnectionState::Disconnected;

                iced::Task::none()
            }
            Event::LoadUsers => {
                let request = GetUsersRequest {};

                let http_client = self.http_client.clone();

                Task::perform(
                    async move { http_client.clone().request(request, None).await },
                    |value| match value {
                        Ok(users) => ui::Event::Chat(Event::AddUsers(users)),
                        Err(err) => ui::Event::ErrorPopup(error_popup::Event::AddApiError(err)),
                    },
                )
            }
            Event::LoadMessages => {
                if let Some(user_id) = self.state.selected_user_id {
                    self.load_messages(user_id)
                } else {
                    event_task(ui::Event::ErrorPopup(error_popup::Event::AddMessage(
                        "Cannot load messages: no user selected".to_string(),
                    )))
                }
            }
            Event::AddUsers(users_response) => {
                let current_user_id = logged_user_data.user_id;
                let users: Vec<User> = users_response
                    .iter()
                    .map(|response| User {
                        id: response.id,
                        username: response.username.clone(),
                    })
                    .filter(|user| user.id != current_user_id)
                    .collect();

                self.state.users = users;
                self.state.next_messages_page += 1;

                Task::none()
            }
            Event::AddMessages(messages_response) => {
                let current_user_id = logged_user_data.user_id;
                let messages: Vec<UserMessage> = messages_response
                    .iter()
                    .map(|response| {
                        if response.user_id == current_user_id {
                            UserMessage::Sent(response.content.clone())
                        } else {
                            UserMessage::Received(response.content.clone())
                        }
                    })
                    .collect();

                self.state.messages.extend(messages);
                self.state.next_messages_page += 1;

                Task::none()
            }
        }
    }

    fn load_messages(&mut self, user_id: i32) -> Task<ui::Event> {
        let pagination = Pagination {
            page: self.state.next_messages_page,
            page_size: 10,
        };
        let request = GetMessagesRequest {
            user_id,
            pagination,
        };

        let http_client = self.http_client.clone();

        Task::perform(
            async move { http_client.clone().request(request, None).await },
            |value| match value {
                Ok(response) => ui::Event::Chat(Event::AddMessages(response)),
                Err(err) => ui::Event::ErrorPopup(error_popup::Event::AddApiError(err)),
            },
        )
    }
}
