use iced::widget::shader::wgpu::Color;
use iced::widget::{
    button, center, column, container, horizontal_space, mouse_area, opaque, row, scrollable, stack, text, text_input, Column, Container
};
use iced::{alignment, Element, Padding, Theme};
use iced::{
    Length::{self},
    widget::vertical_space,
};

use super::{Event, Ui, UserMessage};

impl Ui {
    pub fn get_messages_widget(&self) -> Container<'_, Event> {
        let messages: Element<_> = self
            .messages
            .iter()
            .fold(column![], |col, msg| {
                let row = self.render_message(msg);
                col.push(row)
            })
        .into();

        let scrollable_messages = scrollable(messages)
            .id(self.scroll.clone())
            .height(Length::Fill);

        let scrollable_container = container(column![
            vertical_space().height(Length::Fill),
            scrollable_messages
            .height(Length::Shrink)
            .width(Length::Fill)
        ])
            .style(|_: &Theme| self.theme.scrollable_container);

        return scrollable_container;
    }

    fn render_message(&self, msg: &UserMessage) -> Column<'_, Event> 
    {
        struct RenderData {
            content: String,
            left_portion: u16,
            right_portion: u16
        }

        let message_render_data = match msg {
            UserMessage::Incoming(content) => RenderData {
                content: content.clone(),
                left_portion: 7,
                right_portion: 3,
            },
            UserMessage::Sent(content) => RenderData {
                content: content.clone(),
                left_portion: 3,
                right_portion: 7,
            },
        };

        let text = text(message_render_data.content).size(16.0);
        let bubble = container(text).style(|_| self.theme.message).padding(12);

        column![row![
            container(bubble).width(Length::FillPortion(message_render_data.left_portion)),
            horizontal_space().width(Length::FillPortion(message_render_data.right_portion)),
        ],]
            .width(Length::FillPortion(10))
            .align_x(alignment::Horizontal::Left)
            .padding(Padding {
                top: 0.0,
                right: 30.0,
                bottom: 30.0,
                left: 30.0,
            })
    }

    pub fn get_input_row_widget(&self) -> Container<'_, Event> {
        let message_input = text_input("Type a message...", &self.input_value)
            .on_input(Event::InputChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill)
            .on_submit(Event::InputSubmitted)
            .style(|_, _| self.theme.input);

        let send_button = button("Send")
            .style(|_, _| self.theme.send_btn)
            .on_press(Event::InputSubmitted)
            .padding(10);

        let input_row = container(row![message_input, send_button].spacing(10));

        return input_row;
    }

    pub fn get_error_modal_widget(&self, message: String) {

        todo!()
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)))
            .on_press(on_blur)
        )
    ]
    .into()
}
