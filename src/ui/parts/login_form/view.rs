use iced::{
    Element, Length, alignment,
    widget::{button, column, container, text_input, vertical_space},
};

use super::{Event, Widget};

impl Widget {
    pub fn view(&self) -> Element<Event> {
        let username_input = text_input("Type username...", &self.state.username)
            .on_input(Event::UsernameChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .style(|_, _| self.theme.input);

        let password_input = text_input("Type username...", &self.state.password)
            .on_input(Event::PasswordChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .style(|_, _| self.theme.input);

        let send_button = button("Log in")
            .style(|_, _| self.theme.login_btn)
            .on_press(Event::InputSubmitted)
            .padding(10);

        container(
            column![
                vertical_space().height(Length::Fill),
                username_input,
                password_input,
                send_button,
                vertical_space().height(Length::Fill)
            ]
            .spacing(10)
            .padding(20),
        )
        .align_x(alignment::Horizontal::Center)
        .padding(10)
        .width(Length::Fill)
        .into()
    }
}
