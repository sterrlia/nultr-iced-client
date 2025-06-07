mod config;
mod theme;
mod ui;
mod ws;
mod http;

use ui::Ui;

pub fn main() -> iced::Result {
    config::get_variables();

    iced::application("Chat", Ui::update, Ui::view)
        .subscription(Ui::subscription)
        .run()
}
