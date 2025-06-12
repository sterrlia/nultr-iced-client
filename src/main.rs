mod config;
mod ui;
mod ws;
mod http;
mod auth;

use ui::Ui;

pub fn main() -> iced::Result {
    config::get_variables();

    iced::application("Chat", Ui::update, Ui::view)
        .subscription(Ui::subscription)
        .run()
}
