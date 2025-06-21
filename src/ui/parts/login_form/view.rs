use iced::{
    Element, Length, alignment,
    widget::{button, column, container, text_input},
};

use super::{Event, State, Widget};

impl Widget {
    pub fn view(&self, state: &State) -> Element<Event> {
        let username_input = text_input("Type username", state.username.as_str())
            .on_input(Event::UsernameChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .style(|_, _| self.theme.input);

        let password_input = text_input("Type password", state.password.as_str())
            .on_input(Event::PasswordChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .secure(true)
            .style(|_, _| self.theme.input);

        let send_button = button("Log in")
            .style(|_, _| self.theme.login_btn)
            .on_press(Event::InputSubmitted)
            .padding(10);

        container(
            container(
                column![
                    username_input,
                    password_input,
                    container(send_button)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Right),
                ]
                .align_x(alignment::Horizontal::Center)
                .spacing(10)
                .padding(20),
            )
            .style(|_| self.theme.form_container)
            .width(500),
        )
        .style(|_| self.theme.background)
        .align_y(alignment::Vertical::Center)
        .align_x(alignment::Horizontal::Center)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
