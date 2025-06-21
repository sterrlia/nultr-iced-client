mod view;

use chrono::{NaiveDateTime, Utc};
use client_lib::{
    config,
    util::create_stub_sender,
    ws::{self, controller::SendEvent},
};
use iced::{Element, Task, widget::scrollable};
use rust_api_integrator::http::client::{HttpClient, AuthenticatedHttpClientTrait};
use shared_lib::{request::{
    AuthUserData, GetMessagesRequest, GetMessagesResponse, GetUsersRequest, GetUsersResponse,
    WsMessageRequest,
}, util::MonoResult};
use std::{cmp, sync::Arc};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{ui::{self, theme, AuthSession, WidgetErrorEvent}, util::task_perform};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    SendMessage,
    Reconnect,
    LoadMessages,
    LoadUsers,
    AddUsers(GetUsersResponse),
    AddMessages(GetMessagesResponse),
    SelectUser(i32),
    SendToWs(ws::controller::SendEvent),
}

impl WidgetErrorEvent for Event {
    fn event(self) -> ui::Event {
        ui::Event::Chat(self)
    }

    fn task(self) -> Task<ui::Event> {
        self.event().task()
    }
}

#[derive(Clone, Debug)]
struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Clone, Debug)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct State {
    pub ws_sender: mpsc::UnboundedSender<SendEvent>,
    input_value: String,
    users: Vec<User>,
    messages: Vec<UserMessage>,
    users_scrollable: scrollable::Id,
    messages_scrollable: scrollable::Id,
    selected_user_id: Option<i32>,
    connection_state: ConnectionState,
    messages_page: i32,
}

impl Default for State {
    fn default() -> Self {
        let ws_sender = create_stub_sender::<SendEvent>();

        Self {
            ws_sender,
            input_value: "".to_string(),
            users: Vec::new(),
            messages: Vec::new(),
            selected_user_id: None,
            users_scrollable: scrollable::Id::new("users"),
            messages_scrollable: scrollable::Id::new("messages"),
            connection_state: ConnectionState::Disconnected,
            messages_page: -1,
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
    pub http_client: Arc<HttpClient>,
}

impl Widget {
    pub fn update(
        self: Arc<Self>,
        state: &mut State,
        user_data: AuthUserData,
        event: Event,
    ) -> Task<ui::Event> {
        match event {
            Event::InputChanged(new_value) => {
                state.input_value = new_value;

                iced::Task::none()
            }
            Event::SendMessage => {
                let input_value = state.input_value.trim().to_string();

                if input_value.is_empty() {
                    return iced::Task::none();
                };

                if let Some(user_id) = state.selected_user_id {
                    let uuid = Uuid::new_v4();
                    let content = input_value.clone();
                    let message = UserMessage {
                        uuid,
                        user_id: user_data.user_id,
                        content: content.clone(),
                        created_at: Utc::now().naive_utc(),
                    };

                    state.messages.push(message);

                    state.input_value.clear();

                    let request = WsMessageRequest {
                        id: uuid,
                        user_id,
                        content,
                    };

                    let controller_event = ws::controller::SendEvent::Message(request);

                    Event::SendToWs(controller_event).task()
                } else {
                    error_popup::ErrorEvent::String("User is not chosen".to_string()).task()
                }
            }
            Event::Reconnect => {
                let ws_url = config::get_variables().ws_url.clone();

                let disconnect_event = ws::controller::SendEvent::Disconnect;
                let connect_event = ws::controller::SendEvent::Connect {
                    url: ws_url,
                    token: user_data.token.clone(),
                };

                let disconnect = Event::SendToWs(disconnect_event);
                let connect = Event::SendToWs(connect_event);

                disconnect.task().chain(connect.task())
            }
            Event::LoadUsers => {
                let session = AuthSession {
                    token: user_data.token.clone(),
                };

                task_perform(self.load_users(session))
            }
            Event::LoadMessages => {
                if let Some(user_id) = state.selected_user_id {
                    let session = AuthSession {
                        token: user_data.token.clone(),
                    };
                    state.messages_page += 1;

                    task_perform(self.load_messages(user_id, session, state.messages_page as u64))
                } else {
                    let show_error_event = error_popup::ErrorEvent::String(
                        "Cannot load messages: no user selected".to_string(),
                    )
                    .event();

                    show_error_event.task()
                }
            }
            Event::AddUsers(users_response) => {
                let current_user_id = user_data.user_id;
                let users: Vec<User> = users_response
                    .0
                    .iter()
                    .map(|response| User {
                        id: response.id,
                        username: response.username.clone(),
                    })
                    .filter(|user| user.id != current_user_id)
                    .collect();

                state.users = users;

                Task::none()
            }
            Event::AddMessages(messages_response) => {
                let messages: Vec<UserMessage> = messages_response
                    .0
                    .iter()
                    .cloned()
                    .map(|response| UserMessage {
                        user_id: response.user_id,
                        uuid: response.id,
                        content: response.content,
                        created_at: response.created_at,
                    })
                    .collect();

                state.messages.extend(messages);
                state.messages.sort_by_key(|message| message.created_at);

                state.messages.dedup_by_key(|message| message.uuid);

                Task::none()
            }
            Event::SelectUser(user_id) => {
                state.selected_user_id = Some(user_id);

                if state.messages_page < 0 {
                    Event::LoadMessages.task()
                } else {
                    Task::none()
                }
            }
            Event::SendToWs(event) => {
                if let Err(error) = state.ws_sender.send(event) {
                    tracing::error!("Send error {error}");

                    error_popup::ErrorEvent::String("Unable to connect to server".to_string())
                        .task()
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn ws_update(
        self: Arc<Self>,
        state: &mut State,
        event: ws::controller::Event,
    ) -> Task<ui::Event> {
        match event {
            ws::controller::Event::Ready(ws_sender) => {
                state.ws_sender = ws_sender;

                Task::none()
            }
            ws::controller::Event::Connected => {
                state.connection_state = ConnectionState::Connected;

                Event::LoadUsers.task()
            }
            ws::controller::Event::Message(message_response) => {
                let user_message = UserMessage {
                    uuid: message_response.id,
                    user_id: message_response.user_id,
                    content: message_response.content,
                    created_at: message_response.created_at,
                };

                state.messages.push(user_message);

                iced::Task::none()
            }
            ws::controller::Event::MessageSent => Task::none(),
            ws::controller::Event::Disconnected => {
                state.connection_state = ConnectionState::Disconnected;

                iced::Task::none()
            }
        }
    }

    async fn load_messages(
        self: Arc<Self>,
        user_id: i32,
        session: AuthSession,
        page: u64,
    ) -> MonoResult<ui::Event> {
        let request = GetMessagesRequest {
            user_id,
            page,
            page_size: 20,
        };

        let result = self.http_client.request(request, session.token).await?;
        Ok(match result {
            Ok(response) => Event::AddMessages(response).event(),
            Err(error) => error_popup::ErrorEvent::GetMessages(error).event(),
        })
    }

    async fn load_users(self: Arc<Self>, session: AuthSession) -> MonoResult<ui::Event> {
        let request = GetUsersRequest {};

        let result = self.http_client.request(request, session.token).await?;
        Ok(match result {
            Ok(response) => Event::AddUsers(response).event(),
            Err(error) => error_popup::ErrorEvent::GetUsers(error).event(),
        })
    }
}
