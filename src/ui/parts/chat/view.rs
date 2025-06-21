use iced::{
    Element, Length, Padding, Theme, alignment,
    widget::{
        Button, Column, Container, Svg, button, column, container, horizontal_space, row,
        scrollable, stack, text, text_input, vertical_space,
    },
};
use shared_lib::request::AuthUserData;

use super::{Event, State, User, UserMessage, Widget};

impl Widget {
    pub fn view(&self, state: &State, user_data: AuthUserData) -> Element<Event> {
        let chat_widget = self.get_chat_widget(&state, user_data);
        let user_container = self.get_users_widget(&state);

        container(row![
            user_container
                .width(Length::FillPortion(2))
                .height(Length::Fill),
            chat_widget
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
        user_data: AuthUserData,
    ) -> Container<'_, Event> {
        let input_row = match state.connection_state.clone() {
            super::ConnectionState::Connected => self.get_input_row_widget(state),
            super::ConnectionState::Disconnected => self.get_connect_btn_widget(),
        };
        let message_container = self.get_messages_widget(state, user_data);

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
        user_data: AuthUserData,
    ) -> Container<'_, Event> {
        let messages: Element<_> = state
            .messages
            .iter()
            .fold(column![], |col, msg| {
                let row = self.render_message(msg, user_data.user_id);
                col.push(row)
            })
            .push(vertical_space().height(90))
            .into();

        let scrollable_messages = scrollable(
            container(container(messages).max_width(800))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .id(state.messages_scrollable.clone());

        let scrollable_container = container(column![
            vertical_space().height(Length::Fill),
            scrollable_messages
                .height(Length::Shrink)
                .width(Length::Fill)
        ])
        .style(|_: &Theme| self.theme.message_container);

        return scrollable_container;
    }

    fn render_message(&self, msg: &UserMessage, current_user_id: i32) -> Column<'_, Event> {
        let text = text(msg.content.clone()).size(16.0);
        let bubble = container(
            container(text)
                .style(|_| self.theme.message)
                .width(Length::Shrink)
                .padding(12),
        );

        let row = if msg.user_id == current_user_id {
            row![
                horizontal_space().width(Length::FillPortion(7)),
                bubble
                    .align_x(alignment::Horizontal::Right)
                    .width(Length::FillPortion(3)),
            ]
        } else {
            row![
                bubble
                    .align_x(alignment::Horizontal::Left)
                    .width(Length::FillPortion(3)),
                horizontal_space().width(Length::FillPortion(7)),
            ]
        };

        column![row]
            .width(Length::FillPortion(10))
            .align_x(alignment::Horizontal::Left)
            .padding(Padding {
                top: 0.0,
                right: 30.0,
                bottom: 30.0,
                left: 30.0,
            })
    }

    fn get_users_widget(&self, state: &State) -> Container<'_, Event> {
        let messages: Element<_> = state
            .users
            .iter()
            .fold(column![], |col, user| {
                let row = self.get_user_widget(state.selected_user_id, user);
                col.push(row)
            })
            .into();

        let scrollable_messages = scrollable(messages)
            .id(state.users_scrollable.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        container(
            scrollable_messages
                .height(Length::Shrink)
                .width(Length::Fill),
        )
        .padding(12)
        .align_y(alignment::Vertical::Top)
        .style(|_: &Theme| self.theme.users_container)
    }

    fn get_user_widget(&self, selected_user_id: Option<i32>, user: &User) -> Button<'_, Event> {
        let profile_image_btn = button(Svg::new(self.theme.profile_image_svg.clone()))
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

        let btn_style = if selected_user_id == Some(user.id) {
            self.theme.active_chat_btn
        } else {
            self.theme.chat_btn
        };

        button(user_info_widget)
            .on_press(Event::SelectUser(user.id))
            .width(Length::Fill)
            .style(move |_, _| btn_style)
    }

    pub fn get_input_row_widget(&self, state: &State) -> Container<'_, Event> {
        let message_input = text_input("Type a message...", state.input_value.as_str())
            .on_input(Event::InputChanged)
            .padding(10)
            .size(16)
            .on_submit(Event::SendMessage)
            .style(|_, _| self.theme.input);

        let send_button = button(Svg::new(self.theme.send_btn_svg.clone()))
            .style(|_, _| self.theme.send_btn)
            .on_press(Event::SendMessage);

        let input_row = container(
            row![
                container(message_input).width(Length::Fill),
                send_button.width(35).height(35)
            ]
            .align_y(alignment::Vertical::Center),
        )
        .padding(5)
        .align_x(alignment::Horizontal::Center)
        .style(|_| self.theme.input_container);

        return input_row;
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
    }
}
