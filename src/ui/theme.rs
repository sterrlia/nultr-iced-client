use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{button, container, text_input},
};

pub struct Collection {
    pub app: App,
    pub chat: ChatTheme,
    pub error_popup: ErrorPopup,
    pub login_form: LoginForm,
}

impl Default for Collection {
    fn default() -> Collection {
        let error_popup = ErrorPopup {
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

        let btn = button::Style {
            background: Some(Background::Color(Color::from_rgb(0.4, 0.2, 0.1))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            border: Border {
                radius: Radius::new(10),
                ..Border::default()
            },
            shadow: Shadow::default(),
        };
        let input = text_input::Style {
            border: Border {
                radius: Radius::new(10),
                ..Border::default()
            },
            background: Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
            icon: Color::from_rgb(1.0, 1.0, 1.0),
            placeholder: Color::from_rgb(0.2, 0.2, 0.2),
            value: Color::from_rgb(0.0, 0.0, 0.0),
            selection: Color::from_rgb(0.2, 0.2, 0.2),
        };

        let chat = ChatTheme {
            send_btn: btn,
            chat_btn: btn,
            connect_btn: button::Style {
                background: Some(Background::Color(Color::from_rgb(0.4, 0.2, 0.1))),
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            input,
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
        };

        let login_form = LoginForm {
            login_btn: btn,
            input,
        };

        let app = App {
            background: container::Style {
                text_color: Some(Color::from_rgb(0.7, 0.3, 0.0)),
                background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            }
        };

        Collection {
            app,
            chat,
            error_popup,
            login_form,
        }
    }
}

pub struct ErrorPopup {
    pub message_container: container::Style,
    pub container: container::Style,
    pub close_btn: button::Style,
}

pub struct ChatTheme {
    pub send_btn: button::Style,
    pub connect_btn: button::Style,
    pub chat_btn: button::Style,
    pub input: text_input::Style,
    pub scrollable_container: container::Style,
    pub message: container::Style,
}

pub struct LoginForm {
    pub login_btn: button::Style,
    pub input: text_input::Style,
}

pub struct App {
    pub background: container::Style,
}
