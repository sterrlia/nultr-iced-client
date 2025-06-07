use iced::widget::shader::wgpu::Color;
use iced::widget::{
    Button, Column, Container, Text, button, center, column, container, horizontal_space,
    mouse_area, opaque, row, scrollable, stack, text, text_input,
};
use iced::{Element, Padding, Theme, alignment};
use iced::{
    Length::{self},
    widget::vertical_space,
};

use super::{AppError, Event, Ui, UserMessage};

impl Ui {
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

    pub fn get_error_messages_widget(&self) -> Container<'_, Event> {
        let error_column = self
            .state
            .error_messages
            .iter()
            .enumerate()
            .map(|(index, message)| self.get_error_modal_widget(index, message.clone()))
            .fold(column![vertical_space().height(50)], |column, item| {
                column.push(item)
            });

        let error_column_scrollable = scrollable(
            error_column
                .width(Length::Fill)
                .spacing(10)
                .align_x(alignment::Horizontal::Right),
        )
        .id(self.state.error_messages_scrollable.clone())
        .height(Length::Fill);

        container(
            row![
                horizontal_space().width(50),
                error_column_scrollable,
                horizontal_space().width(50),
            ]
            .width(Length::FillPortion(10)),
        )
        .style(|_| self.theme.error_modal.container)
        .width(700)
        .height(Length::Fill)
    }

    pub fn get_error_modal_widget(&self, index: usize, message: String) -> Container<'_, Event> {
        container(
            button(
                column![text("Error"), text(message),]
                    .spacing(10)
                    .align_x(alignment::Horizontal::Left),
            )
            .on_press(Event::DismissError(index))
            .style(|_, _| self.theme.error_modal.close_btn),
        )
        .padding(10)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(|_| self.theme.error_modal.message_container)
    }

    pub fn get_connect_btn_widget(&self) -> Container<'_, Event> {
        container(
            button(text("Connect"))
                .on_press(Event::Connect)
                .width(Length::Shrink),
        )
        .align_x(alignment::Horizontal::Center)
        .padding(10)
        .width(Length::Fill)
    }
}
