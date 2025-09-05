use iced::{
    Element, Length, alignment,
    widget::{
        Container, button, column, container, horizontal_space, row, scrollable, text,
        vertical_space,
    },
};

use super::{Event, State, Widget};

impl Widget {
    #![allow(mismatched_lifetime_syntaxes)]
    pub fn view(&self, state: &State) -> Element<Event> {
        let error_messages_widget = self.get_error_messages_widget(state);

        container(error_messages_widget)
            .height(Length::Fill)
            .width(Length::FillPortion(10))
            .align_x(alignment::Horizontal::Right)
            .into()
    }

    pub fn get_error_messages_widget(&self, state: &State) -> Container<'_, Event> {
        let error_column = state
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
        .id(state.error_messages_scrollable.clone())
        .height(Length::Fill);

        container(
            row![
                horizontal_space().width(50),
                error_column_scrollable,
                horizontal_space().width(50),
            ]
            .width(Length::FillPortion(10)),
        )
        .style(|_| self.theme.container)
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
            .on_press(Event::DismissMessage(index))
            .style(|_, _| self.theme.close_btn),
        )
        .padding(10)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(|_| self.theme.message_container)
    }
}
