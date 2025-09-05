use iced::{
    Element, Length, Padding, Theme, alignment,
    widget::{
        Button, Column, Container, Svg, button, column, container, horizontal_space, row,
        scrollable, stack, text, text_input, vertical_space,
    },
};
use nultr_shared_lib::request::AuthUserData;

use super::{
    ChatAreaRoomSelectedState, ChatAreaState, ChatMessage, Event, OutgoingMessageState, Room,
    State, User, Widget,
};

impl Widget {
    #![allow(mismatched_lifetime_syntaxes)]
    pub fn view(&self, state: &State, _: AuthUserData) -> Element<Event> {
        let chat_field_widget = match &state.chat_area_state {
            ChatAreaState::RoomNotSelected => self.get_users_widget(state),
            ChatAreaState::RoomSelected(chat_area_state) => {
                self.get_chat_widget(state, chat_area_state)
            }
        };
        let user_container = self.get_rooms_widget(state);

        container(row![
            user_container
                .width(Length::FillPortion(2))
                .height(Length::Fill),
            chat_field_widget
                .width(Length::FillPortion(8))
                .height(Length::Fill)
        ])
        .height(Length::Fill)
        .width(Length::FillPortion(10))
        .align_x(alignment::Horizontal::Center)
        .into()
    }

    pub fn get_chat_widget(
        &self,
        state: &State,
        chat_area_state: &ChatAreaRoomSelectedState,
    ) -> Container<'_, Event> {
        let input_row = match state.connection_state.clone() {
            super::ConnectionState::Connected => self.get_input_row_widget(state),
            super::ConnectionState::Disconnected => self.get_connect_btn_widget(),
        };
        let message_container = self.get_messages_widget(state, chat_area_state);

        container(stack![
            message_container.width(Length::Fill),
            container(input_row.max_width(600))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(alignment::Vertical::Bottom)
                .align_x(alignment::Horizontal::Center)
                .padding(20)
        ])
        .align_x(alignment::Horizontal::Center)
    }

    pub fn get_messages_widget(
        &self,
        state: &State,
        chat_area_state: &ChatAreaRoomSelectedState,
    ) -> Container<'_, Event> {
        let message_widgets: Element<_> = chat_area_state
            .messages
            .iter()
            .fold(column![], |col, msg| {
                let row = self.render_message(msg);
                col.push(row)
            })
            .push(vertical_space().height(90))
            .into();

        let scrollable_messages = scrollable(
            container(container(message_widgets).max_width(800))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .anchor_bottom()
        .id(state.messages_scrollable.clone());

        container(column![
            vertical_space().height(Length::Fill),
            scrollable_messages
                .height(Length::Shrink)
                .width(Length::Fill)
        ])
        .style(|_: &Theme| self.theme.message_container)
    }

    fn render_message(&self, msg: &ChatMessage) -> Column<'_, Event> {
        let get_message_widget = |text| {
            container(text)
                .style(|_| self.theme.message)
                .width(Length::Shrink)
                .padding(12)
        };
        let get_message_container = |message| container(message).width(Length::FillPortion(3));

        let message_space = horizontal_space().width(Length::FillPortion(7));

        let message_row = match msg {
            ChatMessage::Outgoing(message_data) => {
                let message_text = text(message_data.content.clone()).size(16.0);
                let message = get_message_widget(message_text);
                let svg = match message_data.state {
                    OutgoingMessageState::Created => self.theme.message_sent_svg.clone(),
                    OutgoingMessageState::Sent => self.theme.message_sent_svg.clone(),
                    OutgoingMessageState::Received => self.theme.message_received_svg.clone(),
                    OutgoingMessageState::Read => self.theme.message_read_svg.clone(),
                };

                let status_mark_widget = container(
                    container(Svg::new(svg))
                        .center(Length::Fill)
                        .height(17)
                        .width(20),
                )
                .align_x(alignment::Horizontal::Right)
                .align_y(alignment::Vertical::Bottom)
                .height(45)
                .width(Length::Fill);

                row![
                    message_space,
                    get_message_container(stack![
                        message.align_x(alignment::Horizontal::Right),
                        status_mark_widget
                    ])
                    .align_x(alignment::Horizontal::Right)
                ]
            }
            ChatMessage::Incoming(message_data) => {
                let text = text(message_data.content.clone()).size(16.0);

                row![
                    get_message_container(stack![get_message_widget(text)])
                        .align_x(alignment::Horizontal::Left),
                    message_space
                ]
            }
        };

        column![message_row]
            .width(Length::FillPortion(10))
            .padding(Padding {
                top: 0.0,
                right: 30.0,
                bottom: 30.0,
                left: 30.0,
            })
    }

    fn get_rooms_widget(&self, state: &State) -> Container<'_, Event> {
        let rooms: Element<_> = state
            .rooms
            .iter()
            .fold(column![], |col, user| {
                let row = self.get_room_widget(&state.chat_area_state, user);
                col.push(row)
            })
            .into();

        let rooms_scrollable = scrollable(rooms)
            .id(state.rooms_scrollable.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        let show_user_search_btn = self.get_show_user_search_btn_widget();

        container(stack![show_user_search_btn, rooms_scrollable,])
            .padding(12)
            .align_y(alignment::Vertical::Top)
            .style(|_: &Theme| self.theme.rooms_container)
    }

    fn get_room_widget(&self, chat_area_state: &ChatAreaState, room: &Room) -> Button<'_, Event> {
        let profile_image_btn = button(Svg::new(self.theme.profile_image_svg.clone()))
            .height(40)
            .width(40)
            .style(|_, _| self.theme.profile_image_btn);

        let user_info_widget = container(
            row![profile_image_btn, text(room.name.clone())]
                .spacing(10)
                .align_y(alignment::Vertical::Center),
        )
        .padding(5)
        .align_x(alignment::Horizontal::Left);

        let btn_style = match chat_area_state {
            ChatAreaState::RoomSelected(_) => self.theme.active_chat_btn,
            ChatAreaState::RoomNotSelected => self.theme.chat_btn,
        };

        button(user_info_widget)
            .on_press(Event::SelectRoom(room.id))
            .width(Length::Fill)
            .style(move |_, _| btn_style)
    }

    fn get_users_widget(&self, state: &State) -> Container<'_, Event> {
        let users: Element<_> = state
            .users
            .iter()
            .fold(column![], |col, user| {
                let row = self.get_user_widget(user);
                col.push(row)
            })
            .into();

        let users_scrollable = scrollable(users)
            .id(state.rooms_scrollable.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        container(users_scrollable.height(Length::Shrink).width(Length::Fill))
            .padding(12)
            .align_y(alignment::Vertical::Top)
            .style(|_: &Theme| self.theme.users_container)
    }

    fn get_user_widget(&self, user: &User) -> Button<'_, Event> {
        let profile_image_btn =
            button(container(Svg::new(self.theme.profile_image_svg.clone())).center(Length::Fill))
                .height(40)
                .width(40)
                .style(|_, _| self.theme.profile_image_btn);

        let user_info_widget = container(
            row![profile_image_btn, text(user.username.clone())]
                .spacing(10)
                .align_y(alignment::Vertical::Center),
        )
        .padding(5)
        .align_x(alignment::Horizontal::Left);

        button(user_info_widget)
            .on_press(Event::CreatePrivateRoom(user.id))
            .width(Length::Fill)
            .style(move |_, _| self.theme.chat_btn)
    }

    pub fn get_input_row_widget(&self, state: &State) -> Container<'_, Event> {
        let message_input = text_input("Type a message...", state.input_value.as_str())
            .on_input(Event::InputChanged)
            .padding(10)
            .size(16)
            .on_submit(Event::SendMessage)
            .style(|_, _| self.theme.input);

        let send_button =
            button(container(Svg::new(self.theme.send_btn_svg.clone())).center(Length::Fill))
                .style(|_, _| self.theme.send_btn)
                .on_press(Event::SendMessage);

        container(
            row![
                container(message_input).width(Length::Fill),
                send_button.width(35).height(35)
            ]
            .align_y(alignment::Vertical::Center),
        )
        .padding(5)
        .align_x(alignment::Horizontal::Center)
        .style(|_| self.theme.input_container)
    }

    pub fn get_show_user_search_btn_widget(&self) -> Container<'_, Event> {
        container(
            button(container(Svg::new(self.theme.create_room_svg.clone())).center(Length::Fill))
                .style(|_, _| self.theme.show_user_search_btn)
                .on_press(Event::DeselectRoom)
                .height(35)
                .width(35),
        )
        .align_x(alignment::Horizontal::Right)
        .align_y(alignment::Vertical::Bottom)
        .width(Length::Fill)
        .height(Length::Fill)
    }

    pub fn get_connect_btn_widget(&self) -> Container<'_, Event> {
        container(
            button(text("Connect"))
                .style(|_, _| self.theme.connect_btn)
                .on_press(Event::Reconnect)
                .width(Length::Shrink),
        )
        .align_x(alignment::Horizontal::Center)
        .padding(10)
        .width(Length::Fill)
        .style(|_| self.theme.input_container)
    }
}
