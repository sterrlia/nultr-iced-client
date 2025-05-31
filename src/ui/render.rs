use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, stack, text, text_input, Column, Container, Row, Text
};
use iced::{alignment, Element, Padding, Subscription, Task, Theme};
use iced::{
    Length::{self},
    widget::vertical_space,
};

use super::{Event, Ui};

impl Ui {
    pub fn view(&self) -> Element<Event> {
        let scrollable_container = self.get_messages_widget();
        let input_row = self.get_input_row_widget();

        stack! [
        container(
            column![
            scrollable_container.width(Length::Fixed(600.0)),
            input_row.width(Length::Fixed(600.0)),
            ]
            .padding(20)
            .spacing(20),
        )
            .style(|_: &Theme| self.theme.background)
            .height(Length::Fill)
            .width(Length::FillPortion(10))
            .align_x(alignment::Horizontal::Center),

            container(
                column![
                    button("Send")
                    .style(|_, _| self.theme.send_btn)
                    .on_press(Event::InputSubmitted)
                    .padding(10)
                ]

                .padding(20)
                .spacing(20),
            )
                .height(Length::Fill)
                .width(Length::FillPortion(10))
                .align_x(alignment::Horizontal::Right),

        ].into()
    }
} 
