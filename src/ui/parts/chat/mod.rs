mod view;

use chrono::{NaiveDateTime, Utc};
use iced::{Element, Task, widget::scrollable};
use nultr_client_lib::{
    config,
    util::create_stub_sender,
    ws::{self, controller::SendEvent},
};
use nultr_shared_lib::{
    request::{
        AuthUserData, CreatePrivateRoomRequest, CreatePrivateRoomResponse, GetMessagesRequest,
        GetMessagesResponse, GetRoomsRequest, GetRoomsResponse, GetUsersRequest, GetUsersResponse,
        Identifier, MessageResponse, UuidIdentifier, WsMarkMessagesReadRequest, WsMessageRequest,
    },
    util::MonoResult,
};
use rust_api_kit::http::client::{AuthenticatedHttpClientTrait, BearerToken, HttpClient};
use std::{cmp, sync::Arc};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    ui::{self, WidgetErrorEvent, theme},
    util::task_perform,
};

use super::error_popup;

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    DeselectRoom,
    SendMessage,
    Reconnect,
    CreatePrivateRoom(Identifier),
    LoadMessages,
    LoadUsers,
    LoadRooms,
    AddCreatedRoom(CreatePrivateRoomResponse),
    AddRooms(GetRoomsResponse),
    AddUsers(GetUsersResponse),
    AddMessages(GetMessagesResponse),
    SelectRoom(Identifier),
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
pub struct User {
    pub id: Identifier,
    pub username: String,
}

#[derive(Clone, Debug)]
struct Room {
    pub id: Identifier,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum ChatMessage {
    Outgoing(OutgoingChatMessage),
    Incoming(IncomingChatMessage),
}

#[derive(Clone, Debug)]
pub struct IncomingChatMessage {
    pub user_id: Identifier,
    pub uuid: UuidIdentifier,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct OutgoingChatMessage {
    pub user_id: Identifier,
    pub uuid: UuidIdentifier,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub state: OutgoingMessageState,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum OutgoingMessageState {
    Created = 1,
    Sent = 2,
    Received = 3,
    Read = 4,
}

#[derive(Clone, Debug)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Clone, Debug)]
pub enum ChatAreaState {
    RoomSelected(ChatAreaRoomSelectedState),
    RoomNotSelected,
}

#[derive(Clone, Debug)]
pub struct ChatAreaRoomSelectedState {
    pub room_id: Identifier,
    pub messages: Vec<ChatMessage>,
    pub messages_page: i32,
}

#[derive(Clone, Debug)]
pub struct State {
    pub ws_sender: mpsc::UnboundedSender<SendEvent>,
    input_value: String,
    rooms: Vec<Room>,
    chat_area_state: ChatAreaState,
    users: Vec<User>,
    rooms_scrollable: scrollable::Id,
    messages_scrollable: scrollable::Id,
    connection_state: ConnectionState,
}

impl Default for State {
    fn default() -> Self {
        let ws_sender = create_stub_sender::<SendEvent>();

        Self {
            ws_sender,
            input_value: "".to_string(),
            rooms: Vec::new(),
            chat_area_state: ChatAreaState::RoomNotSelected,
            users: Vec::new(),
            rooms_scrollable: scrollable::Id::new("users"),
            messages_scrollable: scrollable::Id::new("messages"),
            connection_state: ConnectionState::Disconnected,
        }
    }
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
            Event::CreatePrivateRoom(user_id) => {
                task_perform(self.create_room(user_data.token.clone(), user_id))
            }
            Event::SendMessage => match &mut state.chat_area_state {
                ChatAreaState::RoomSelected(chat_area_state) => {
                    let input_value = state.input_value.trim().to_string();

                    if input_value.is_empty() {
                        return Task::none();
                    };

                    let uuid = Uuid::new_v4();
                    let content = input_value.clone();
                    let message = ChatMessage::Outgoing(OutgoingChatMessage {
                        uuid,
                        user_id: user_data.user_id,
                        content: content.clone(),
                        created_at: Utc::now().naive_utc(),
                        state: OutgoingMessageState::Created,
                    });

                    chat_area_state.messages.push(message);

                    state.input_value.clear();

                    let request = WsMessageRequest {
                        uuid,
                        room_id: chat_area_state.room_id,
                        content,
                    };

                    let controller_event = ws::controller::SendEvent::Message(request);

                    Event::SendToWs(controller_event).task()
                }
                ChatAreaState::RoomNotSelected => {
                    error_popup::ErrorEvent::String("User is not chosen".to_string()).task()
                }
            },
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
            Event::LoadUsers => task_perform(self.load_users(user_data.token.clone())),
            Event::LoadRooms => task_perform(self.load_rooms(user_data.token.clone())),
            Event::LoadMessages => match &state.chat_area_state {
                ChatAreaState::RoomSelected(chat_area_state) => task_perform(self.load_messages(
                    chat_area_state.room_id,
                    user_data.token.clone(),
                    chat_area_state.messages_page as u64,
                )),
                ChatAreaState::RoomNotSelected => error_popup::ErrorEvent::String(
                    "Cannot load messages: no user selected".to_string(),
                )
                .task(),
            },
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
            Event::AddMessages(messages_response) => match &mut state.chat_area_state {
                ChatAreaState::RoomSelected(chat_area_state) => {
                    let from_response_to_message_fn = |response: MessageResponse| {
                        if user_data.user_id == response.user_id {
                            let state = if response.read {
                                OutgoingMessageState::Read
                            } else {
                                OutgoingMessageState::Received
                            };

                            ChatMessage::Outgoing(OutgoingChatMessage {
                                user_id: response.user_id,
                                uuid: response.uuid,
                                content: response.content,
                                created_at: response.created_at,
                                state,
                            })
                        } else {
                            ChatMessage::Incoming(IncomingChatMessage {
                                user_id: response.user_id,
                                uuid: response.uuid,
                                content: response.content,
                                created_at: response.created_at,
                            })
                        }
                    };

                    let new_messages: Vec<ChatMessage> = messages_response
                        .0
                        .iter()
                        .cloned()
                        .map(from_response_to_message_fn)
                        .collect();

                    chat_area_state.messages.extend(new_messages);
                    chat_area_state
                        .messages
                        .sort_by_key(|message| match message {
                            ChatMessage::Outgoing(message) => message.created_at,
                            ChatMessage::Incoming(message) => message.created_at,
                        });

                    chat_area_state
                        .messages
                        .dedup_by_key(|message| match message {
                            ChatMessage::Outgoing(message) => message.uuid,
                            ChatMessage::Incoming(message) => message.uuid,
                        });

                    chat_area_state.messages_page += 1;

                    let unread_message_uuids: Vec<UuidIdentifier> = messages_response
                        .0
                        .iter()
                        .filter(|message| !message.read)
                        .map(|message| message.uuid)
                        .collect();

                    // TODO: read only visible
                    let ws_request = WsMarkMessagesReadRequest {
                        room_id: chat_area_state.room_id,
                        message_uuids: unread_message_uuids,
                    };

                    Event::SendToWs(ws::controller::SendEvent::MessagesRead(ws_request)).task()
                }
                ChatAreaState::RoomNotSelected => Task::none(),
            },
            Event::AddRooms(get_rooms_response) => {
                let rooms: Vec<Room> = get_rooms_response
                    .0
                    .iter()
                    .map(|response| Room {
                        id: response.id,
                        name: response.name.clone(),
                    })
                    .collect();

                state.rooms = rooms;

                Task::none()
            }
            Event::AddCreatedRoom(room_response) => {
                let room = Room {
                    id: room_response.id,
                    name: room_response.name,
                };
                state.rooms.push(room);

                Task::none()
            }
            Event::SelectRoom(room_id) => match state.chat_area_state.clone() {
                ChatAreaState::RoomSelected(chat_area_state) => {
                    if chat_area_state.room_id == room_id {
                        Task::none()
                    } else {
                        state.chat_area_state =
                            ChatAreaState::RoomSelected(ChatAreaRoomSelectedState {
                                room_id,
                                messages: Vec::new(),
                                messages_page: 0,
                            });

                        Event::LoadMessages.task()
                    }
                }
                ChatAreaState::RoomNotSelected => {
                    state.chat_area_state =
                        ChatAreaState::RoomSelected(ChatAreaRoomSelectedState {
                            room_id,
                            messages: Vec::new(),
                            messages_page: 0,
                        });

                    Event::LoadMessages.task()
                }
            },
            Event::SendToWs(event) => {
                if let Err(error) = state.ws_sender.send(event) {
                    tracing::error!("Send error {error}");

                    error_popup::ErrorEvent::String("Unable to connect to server".to_string())
                        .task()
                } else {
                    Task::none()
                }
            }
            Event::DeselectRoom => {
                state.chat_area_state = ChatAreaState::RoomNotSelected;

                Task::none()
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

                Event::LoadUsers.task().chain(Event::LoadRooms.task())
            }
            ws::controller::Event::Message(message_response) => match &mut state.chat_area_state {
                // TODO: change page every page_size
                ChatAreaState::RoomSelected(chat_area_state) => {
                    let uuid = message_response.uuid;
                    let user_message = ChatMessage::Incoming(IncomingChatMessage {
                        uuid,
                        user_id: message_response.user_id,
                        content: message_response.content,
                        created_at: message_response.created_at,
                    });

                    chat_area_state.messages.push(user_message);

                    let ws_request = WsMarkMessagesReadRequest {
                        room_id: chat_area_state.room_id,
                        message_uuids: vec![uuid],
                    };

                    Event::SendToWs(ws::controller::SendEvent::MessagesRead(ws_request)).task()
                }
                ChatAreaState::RoomNotSelected => Task::none(),
            },
            ws::controller::Event::MessageSent(message_uuid) => {
                Self::change_outgoing_messages_state(
                    state,
                    vec![message_uuid],
                    OutgoingMessageState::Sent,
                );

                Task::none()
            }
            ws::controller::Event::MessageReceived(message_uuid) => {
                Self::change_outgoing_messages_state(
                    state,
                    vec![message_uuid],
                    OutgoingMessageState::Received,
                );

                Task::none()
            }
            ws::controller::Event::MessagesRead(response) => {
                if let ChatAreaState::RoomSelected(chat_area_state) = &state.chat_area_state {
                    if chat_area_state.room_id != response.room_id {
                        return Task::none();
                    }
                }

                Self::change_outgoing_messages_state(
                    state,
                    response.message_uuids,
                    OutgoingMessageState::Read,
                );

                Task::none()
            }
            ws::controller::Event::Disconnected => {
                state.connection_state = ConnectionState::Disconnected;

                Task::none()
            }
        }
    }

    fn change_outgoing_messages_state(
        state: &mut State,
        message_uuids: Vec<UuidIdentifier>,
        message_state: OutgoingMessageState,
    ) {
        match &mut state.chat_area_state {
            ChatAreaState::RoomSelected(chat_area_state) => {
                let target_messages: Vec<&mut OutgoingChatMessage> = chat_area_state
                    .messages
                    .iter_mut()
                    .filter_map(|message| {
                        if let ChatMessage::Outgoing(message) = message {
                            Some(message)
                        } else {
                            None
                        }
                    })
                    .filter(|message| message_uuids.contains(&message.uuid))
                    .collect();

                for message in target_messages {
                    if (message.state.clone() as u8) < (message_state.clone() as u8) {
                        message.state = message_state.clone();
                    }
                }
            }
            ChatAreaState::RoomNotSelected => (),
        }
    }

    async fn load_messages(
        self: Arc<Self>,
        room_id: Identifier,
        session_token: BearerToken,
        page: u64,
    ) -> MonoResult<ui::Event> {
        let request = GetMessagesRequest {
            room_id,
            page,
            page_size: 20,
        };

        let result = self.http_client.request(request, session_token).await?;

        Ok(match result {
            Ok(response) => Event::AddMessages(response).event(),
            Err(error) => error_popup::ErrorEvent::GetMessages(error).event(),
        })
    }

    async fn load_users(self: Arc<Self>, session_token: BearerToken) -> MonoResult<ui::Event> {
        let request = GetUsersRequest {};

        let result = self.http_client.request(request, session_token).await?;

        Ok(match result {
            Ok(response) => Event::AddUsers(response).event(),
            Err(error) => error_popup::ErrorEvent::GetUsers(error).event(),
        })
    }

    async fn load_rooms(self: Arc<Self>, session_token: BearerToken) -> MonoResult<ui::Event> {
        let request = GetRoomsRequest {};

        let result = self.http_client.request(request, session_token).await?;

        Ok(match result {
            Ok(response) => Event::AddRooms(response).event(),
            Err(error) => error_popup::ErrorEvent::GetRooms(error).event(),
        })
    }

    async fn create_room(
        self: Arc<Self>,
        session_token: BearerToken,
        user_id: Identifier,
    ) -> MonoResult<ui::Event> {
        let request = CreatePrivateRoomRequest {
            receiver_user_id: user_id,
            name: None,
        };

        let result = self.http_client.request(request, session_token).await?;

        Ok(match result {
            Ok(response) => Event::AddCreatedRoom(response).event(),
            Err(error) => error_popup::ErrorEvent::CreateRoom(error).event(),
        })
    }
}
