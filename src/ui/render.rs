use iced::widget::{
    Column, Container, Row, Text, button, column, container, horizontal_space, row, scrollable,
    stack, text, text_input,
};
use iced::{Element, Padding, Subscription, Task, Theme, alignment};
use iced::{
    Length::{self},
    widget::vertical_space,
};

use super::{Event, Ui};

impl Ui {
    pub fn view(&self) -> Element<Event> {
        let scrollable_container = self.get_messages_widget();
        let input_row = match self.state.connection_state {
            super::ConnectionState::Connected => self.get_input_row_widget(),
            super::ConnectionState::Disconnected => self.get_connect_btn_widget(),
        };

        let error_messages_widget = self.get_error_messages_widget();

        let chat_widget = container(
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
            .align_x(alignment::Horizontal::Center);

        let notification_widget = container(error_messages_widget)
            .height(Length::Fill)
            .width(Length::FillPortion(10))
            .align_x(alignment::Horizontal::Right);

        stack![
            chat_widget,
            notification_widget
        ]
        .into()
    }
}
