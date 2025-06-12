use iced::Task;

use crate::http;

use super::Event;

pub fn event_task(event: Event) -> Task<Event> {
    Task::perform(async { event }, |value| value)
}
