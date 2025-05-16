use chat::Chat;

mod api;
mod app_theme;
mod chat;

pub fn main() -> iced::Result {
    iced::run("Chat", Chat::update, Chat::view)
}
