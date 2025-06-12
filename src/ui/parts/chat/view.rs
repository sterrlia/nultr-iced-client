use iced::{
    Element, Length, Padding, Theme, alignment,
    widget::{
        Column, Container, button, column, container, horizontal_space, row, scrollable, text,
        text_input, vertical_space,
    },
};

use crate::ui;

use super::{Event, UserMessage, Widget};

impl Widget {
    pub fn view(&self) -> Element<Event> {
        let scrollable_container = self.get_messages_widget();
        let input_row = match self.state.connection_state {
            super::ConnectionState::Connected => self.get_input_row_widget(),
            super::ConnectionState::Disconnected => self.get_connect_btn_widget(),
        };

        container(
            column![
                scrollable_container.width(Length::Fixed(600.0)),
                input_row.width(Length::Fixed(600.0)),
            ]
            .padding(20)
            .spacing(20),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(10))
        .align_x(alignment::Horizontal::Center)
        .into()
    }

    pub fn get_messages_widget(&self) -> Container<'_, Event> {
        let messages: Element<_> = self
            .state
            .messages
            .iter()
            .fold(column![], |col, msg| {
                let row = self.render_message(msg);
                col.push(row)
            })
            .into();

        let scrollable_messages = scrollable(messages)
            .id(self.state.messages_scrollable.clone())
            .height(Length::Fill);

        let scrollable_container = container(column![
            vertical_space().height(Length::Fill),
            scrollable_messages
                .height(Length::Shrink)
                .width(Length::Fill)
        ])
        .style(|_: &Theme| self.theme.scrollable_container);

        return scrollable_container;
    }

    fn render_message(&self, msg: &UserMessage) -> Column<'_, Event> {
        struct RenderData {
            content: String,
            left_portion: u16,
            right_portion: u16,
        }

        let message_render_data = match msg {
            UserMessage::Received(content) => RenderData {
                content: content.clone(),
                left_portion: 7,
                right_portion: 3,
            },
            UserMessage::Sent(content) => RenderData {
                content: content.clone(),
                left_portion: 3,
                right_portion: 7,
            },
        };

        let text = text(message_render_data.content).size(16.0);
        let bubble = container(text).style(|_| self.theme.message).padding(12);

        column![row![
            container(bubble).width(Length::FillPortion(message_render_data.left_portion)),
            horizontal_space().width(Length::FillPortion(message_render_data.right_portion)),
        ],]
        .width(Length::FillPortion(10))
        .align_x(alignment::Horizontal::Left)
        .padding(Padding {
            top: 0.0,
            right: 30.0,
            bottom: 30.0,
            left: 30.0,
        })
    }

    pub fn get_input_row_widget(&self) -> Container<'_, Event> {
        let message_input = text_input("Type a message...", &self.state.input_value)
            .on_input(Event::InputChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .on_submit(Event::SendMessage)
            .style(|_, _| self.theme.input);

        let send_button = button("Send")
            .style(|_, _| self.theme.send_btn)
            .on_press(Event::SendMessage)
            .padding(10);

        let input_row = container(row![message_input, send_button].spacing(10));

        return input_row;
    }

    pub fn get_connect_btn_widget(&self) -> Container<'_, Event> {
        container(
            button(text("Connect"))
                .on_press(Event::Reconnect)
                .width(Length::Shrink),
        )
        .align_x(alignment::Horizontal::Center)
        .padding(10)
        .width(Length::Fill)
    }
}
