use iced::Task;
use shared_lib::util::MonoResult;

pub fn task_perform<F, T>(future: F) -> Task<T>
where
    F: Future<Output = MonoResult<T>> + Send + 'static,
    T: Send + 'static,
{
    Task::perform(future, |result| match result {
        Ok(value) => value,
        Err(value) => value,
    })
}
