use iced::widget::container;
use iced::{Element, widget::stack};

use super::{AuthState, Event, Ui};

impl Ui {
    #![allow(mismatched_lifetime_syntaxes)]
    pub fn view(&self) -> Element<Event> {
        let error_popup = self
            .error_popup
            .view(&self.state.error_popup)
            .map(Event::ErrorPopup);

        let page = match self.auth_state.clone() {
            AuthState::Authenticated(user_data) => {
                self.chat.view(&self.state.chat, user_data).map(Event::Chat)
            }
            AuthState::Unauthenticated => self
                .login
                .view(&self.state.login_form)
                .map(Event::LoginForm),
        };

        container(stack![page, error_popup])
            .style(|_| self.theme.background)
            .into()
    }
}
