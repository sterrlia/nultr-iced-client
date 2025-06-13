mod view;

use chrono::{NaiveDateTime, Utc};
use iced::{Element, Task, widget::scrollable};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth, config,
    http::{
        self,
        models::{GetMessagesRequest, GetUsersRequest, MessageResponse, Pagination, UserResponse},
    },
    ui::{self, theme, util::event_task},
    ws,
};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    SendMessage,
    Message(UserMessage),
    MessageSent,
    Reconnect,
    Connected,
    Disconnected,
    LoadMessages,
    LoadUsers,
    AddUsers(Vec<UserResponse>),
    AddMessages(Vec<MessageResponse>),
    SelectUser(i32),
    LoadMessagesError(http::api::Error<http::models::ErrorResponse>)
}

#[derive(Clone, Debug)]
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
    users_scrollable: scrollable::Id,
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
            users_scrollable: scrollable::Id::new("users"),
            messages_scrollable: scrollable::Id::new("messages"),
            connection_state: ConnectionState::Disconnected,
            next_messages_page: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserMessage {
    pub user_id: i32,
    pub uuid: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
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

                if input_value.is_empty() {
                    return iced::Task::none();
                };

                if let Some(user_id) = self.state.selected_user_id {
                    let uuid = Uuid::new_v4();
                    let content = input_value.clone();
                    let message = UserMessage {
                        uuid,
                        user_id: logged_user_data.user_id,
                        content: content.clone(),
                        created_at: Utc::now().naive_utc(),
                    };

                    self.state.messages.push(message);

                    self.state.input_value.clear();

                    let request = ws::client::MessageRequest {
                        uuid,
                        user_id,
                        content,
                    };

                    let controller_event = ws::controller::SendEvent::Message(request);

                    event_task(ui::Event::ToWs(controller_event))
                } else {
                    event_task(ui::Event::ErrorPopup(error_popup::Event::AddMessage(
                        "User is not chosen".to_string(),
                    )))
                }
            }
            Event::Message(message) => {
                self.state.messages.push(message);

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

                let disconnect = ui::Event::ToWs(disconnect_event);
                let connect = ui::Event::ToWs(connect_event);

                event_task(disconnect).chain(event_task(connect))
            }
            Event::Connected => {
                self.state.connection_state = ConnectionState::Connected;

                let get_users = ui::Event::Chat(Event::LoadUsers);

                event_task(get_users)
            }
            Event::Disconnected => {
                self.state.connection_state = ConnectionState::Disconnected;

                iced::Task::none()
            }
            Event::LoadUsers => {
                let request = GetUsersRequest {};

                let http_client = self.http_client.clone();
                let session = http::models::Session {
                    token: logged_user_data.token,
                };

                Task::perform(
                    async move { http_client.clone().request(request, Some(session)).await },
                    |value| match value {
                        Ok(users) => ui::Event::Chat(Event::AddUsers(users)),
                        Err(err) => ui::Event::ErrorPopup(error_popup::Event::AddApiError(err)),
                    },
                )
            }
            Event::LoadMessages => {
                if let Some(user_id) = self.state.selected_user_id {
                    let session = http::models::Session {
                        token: logged_user_data.token,
                    };

                    self.load_messages(user_id, session)
                } else {
                    let show_error_event = ui::Event::ErrorPopup(error_popup::Event::AddMessage(
                        "Cannot load messages: no user selected".to_string(),
                    ));

                    event_task(show_error_event)
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

                Task::none()
            }
            Event::AddMessages(messages_response) => {
                let messages: Vec<UserMessage> = messages_response
                    .iter()
                    .cloned()
                    .map(|response| UserMessage {
                        user_id: response.user_id,
                        uuid: response.id,
                        content: response.content,
                        created_at: response.created_at,
                    })
                    .collect();

                self.state.messages.extend(messages);
                self.state
                    .messages
                    .sort_by_key(|message| message.created_at);
                self.state.messages.dedup_by_key(|message| message.uuid);

                self.state.next_messages_page += 1;

                Task::none()
            }
            Event::SelectUser(user_id) => {
                self.state.selected_user_id = Some(user_id);

                event_task(ui::Event::Chat(Event::LoadMessages))
            }
            Event::LoadMessagesError(error) => {
                self.state.selected_user_id = None;

                let show_error = ui::Event::ErrorPopup(error_popup::Event::AddApiError(error));

                event_task(show_error)
            }
        }
    }

    fn load_messages(&mut self, user_id: i32, session: http::models::Session) -> Task<ui::Event> {
        let pagination = Pagination {
            page: self.state.next_messages_page as u64,
            page_size: 10,
        };
        let request = GetMessagesRequest {
            user_id,
            pagination,
        };

        let http_client = self.http_client.clone();

        Task::perform(
            async move { http_client.clone().request(request, Some(session)).await },
            |value| match value {
                Ok(response) => ui::Event::Chat(Event::AddMessages(response)),
                Err(err) => ui::Event::Chat(Event::LoadMessagesError(err))
            },
        )
    }
}
