use iced::widget::container;
use iced::{Element, widget::stack};

use super::{AuthState, Event, Ui};

impl Ui {
    pub fn view(&self) -> Element<Event> {
        let error_popup = self.error_popup.view().map(Event::ErrorPopup);
        let page = match self.state.auth_state {
            AuthState::Authorized(_) => self.chat.view().map(Event::Chat),
            AuthState::Unauthorized => self.login.view().map(Event::LoginForm),
        };

        container(stack![page, error_popup])
            .style(|_| self.theme.background)
            .into()
    }
}
