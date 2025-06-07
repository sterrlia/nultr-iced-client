use super::Event;

pub fn event_task(event: Event) -> iced::Task<Event> {
    iced::Task::perform(async { event }, |value| value)
}
