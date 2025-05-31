mod config;
mod theme;
mod ui;
mod controller;

use ui::Ui;


pub fn main() -> iced::Result {
    iced::application("Chat", Ui::update, Ui::view)
        .subscription(Ui::subscription)
        .run()
}
