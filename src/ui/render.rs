use iced::widget::{
    Column, Container, Row, Text, button, column, container, horizontal_space, row, scrollable,
    text, text_input,
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
            .align_x(alignment::Horizontal::Center)
            .into()
    }
} 
