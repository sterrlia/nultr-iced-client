use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{button, container, text_input},
};

pub struct AppTheme {
    pub chat: ChatTheme,
}

impl Default for AppTheme {
    fn default() -> AppTheme {
        let error_modal =  ErrorModalStyle { 
            message_container: container::Style {
                background: Some(Background::Color([0.4, 0.0, 0.0, 0.7].into())),
                border: Border {
                    radius: Radius::new(10.0),
                    ..Border::default()
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            },
            container: container::Style {
                text_color: None,
                background: None,
                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            },
            close_btn: button::Style {
                background: None,
                ..button::Style::default()
            },
        };

        let chat = ChatTheme {
            send_btn: button::Style {
                background: Some(Background::Color(Color::from_rgb(0.4, 0.2, 0.1))),
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            connect_btn: button::Style {
                background: Some(Background::Color(Color::from_rgb(0.4, 0.2, 0.1))),
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            input: text_input::Style {
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                background: Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
                icon: Color::from_rgb(1.0, 1.0, 1.0),
                placeholder: Color::from_rgb(0.2, 0.2, 0.2),
                value: Color::from_rgb(0.0, 0.0, 0.0),
                selection: Color::from_rgb(0.2, 0.2, 0.2),
            },
            scrollable_container: container::Style {
                text_color: Some(Color::from_rgb(200.0, 44.0, 0.0)),
                background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    radius: Radius::new(15),
                    width: 5.0,
                },
                ..container::Style::default()
            },
            message: container::Style {
                text_color: Some(Color::from_rgb(200.0, 44.0, 0.0)),
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
                border: Border {
                    radius: Radius::new(15),
                    ..Border::default()
                },
                ..container::Style::default()
            },
            background: container::Style {
                text_color: Some(Color::from_rgb(0.7, 0.3, 0.0)),
                background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            },
            error_modal
        };

        AppTheme { chat }
    }
}

pub struct ErrorModalStyle {
    pub message_container: container::Style,
    pub container: container::Style,
    pub close_btn: button::Style
}

pub struct ChatTheme {
    pub send_btn: button::Style,
    pub connect_btn: button::Style,
    pub input: text_input::Style,
    pub scrollable_container: container::Style,
    pub message: container::Style,
    pub background: container::Style,
    pub error_modal: ErrorModalStyle
}
