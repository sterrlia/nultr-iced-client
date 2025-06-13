use iced::widget::container;
use iced::{Element, widget::stack};

use super::{AuthState, Event, Ui};

impl Ui {
    pub fn view(&self) -> Element<Event> {
        let error_popup = self.error_popup.view().map(Event::ErrorPopup);
        let page = match self.state.auth_state.clone() {
            AuthState::Authenticated(user_data) => self.chat.view(user_data).map(Event::Chat),
            AuthState::Unauthenticated => self.login.view().map(Event::LoginForm),
        };

        container(stack![page, error_popup])
            .style(|_| self.theme.background)
            .into()
    }
}
