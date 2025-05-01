use chat::Chat;

mod api;
mod app_theme;
mod chat;

pub fn main() -> iced::Result {
    iced::run("A cool counter", Chat::update, Chat::view)
}
