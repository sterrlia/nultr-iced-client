use iced::widget::{
    Column, Container, Row, Text, button, column, container, horizontal_space, row, scrollable,
    text, text_input,
};
use iced::{
    alignment, Element, Padding, Theme
};
use iced::{
    Length::{self},
    widget::vertical_space,
};

use crate::app_theme::{AppTheme, ChatTheme};

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SendPressed,
}

pub struct Chat {
    theme: ChatTheme,
    input_value: String,
    messages: Vec<String>,
    scroll: scrollable::Id,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            theme: AppTheme::default().chat,
            input_value: "".to_string(),
            messages: Vec::new(),
            scroll: scrollable::Id::new("1".to_string()),
        }
    }
}

impl Chat {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(new_value) => {
                self.input_value = new_value;
            }
            Message::SendPressed => {
                if !self.input_value.trim().is_empty() {
                    self.messages.push(self.input_value.trim().to_string());
                    self.input_value.clear();
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let scrollable_container = self.get_messages_widget();
        let input_row = self.get_input_row_widget();

        container(
            column![
                scrollable_container.width(Length::Fixed(600.0)),
                input_row.width(Length::Fixed(600.0)),
            ]
            .padding(20)
            .spacing(20)
        )
        .style(|_: &Theme| self.theme.background)
        .height(Length::Fill)
        .width(Length::FillPortion(10))
        .align_x(alignment::Horizontal::Center)
        .into()
    }

    fn get_messages_widget(&self) -> Container<'_, Message> {
        let messages: Element<_> = self
            .messages
            .iter()
            .fold(column![], |col, msg| {
                let text = text(msg).size(16.0);
                let bubble = container(text).style(|_| self.theme.message).padding(12);

                let row = column![row![
                    container(bubble).width(Length::FillPortion(7)),
                    horizontal_space().width(Length::FillPortion(3)),
                ],]
                    .width(Length::FillPortion(10))
                    .align_x(alignment::Horizontal::Left)
                    .padding(Padding {
                        top: 0.0,
                        right: 30.0,
                        bottom: 30.0,
                        left: 30.0,
                    });

                col.push(row)
            })
        .into();

        let scrollable_messages = scrollable(messages)
            .id(self.scroll.clone())
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

    fn get_input_row_widget(&self) -> Container<'_, Message> {
        let message_input = text_input("Type a message...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .on_submit(Message::SendPressed)
            .style(|_, _| self.theme.input);

        let send_button = button("Send")
            .style(|_, _| self.theme.send_btn)
            .on_press(Message::SendPressed)
            .padding(10);

        let input_row = container(row![message_input, send_button].spacing(10));

        return input_row;
    }
}
