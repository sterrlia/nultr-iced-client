mod ui;
mod util;
use nultr_client_lib::config;
use ui::Ui;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    config::get_variables();

    iced::application("Nultr", Ui::update, Ui::view)
        .subscription(Ui::subscription)
        .run()
}
