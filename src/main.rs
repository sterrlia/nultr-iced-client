mod ui;
mod util;
use client_lib::config;
use ui::Ui;
use tracing_subscriber;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    config::get_variables();

    iced::application("Chat", Ui::update, Ui::view)
        .subscription(Ui::subscription)
        .run()
}
