use iced::{
    Background, Border, Color, Shadow,
    border::Radius,
    widget::{Svg, button, container, svg, text_input},
};
use nultr_procmacro_lib::{color, svg_handle};

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

        let input = text_input::Style {
            border: Border {
                radius: Radius::new(10),
                ..Border::default()
            },
            background: Background::Color(color!("#303030")),
            icon: color!("#D3D3D3"),
            placeholder: color!("#505050"),
            value: color!("#D3D3D3"),
            selection: color!("#3584E4"),
        };

        let chat_btn = button::Style {
            background: Some(Background::Color(color!("#181818"))),
            text_color: color!("#D3D3D3"),
            border: Border {
                radius: Radius::new(10),
                ..Border::default()
            },
            ..button::Style::default()
        };
        let chat = ChatTheme {
            send_btn_svg: svg_handle!("arrow-up-from-dot"),
            create_room_svg: svg_handle!("plus"),
            message_sent_svg: svg_handle!("sent"),
            message_read_svg: svg_handle!("read"),
            message_received_svg: svg_handle!("received"),
            profile_image_svg: svg_handle!("user"),
            show_user_search_btn: button::Style {
                background: Some(Background::Color(color!("#D3D3D3"))),
                text_color: color!("#000000"),
                border: Border {
                    radius: Radius::new(100),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            send_btn: button::Style {
                background: Some(Background::Color(color!("#D3D3D3"))),
                text_color: color!("#000000"),
                border: Border {
                    radius: Radius::new(100),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            chat_btn,
            active_chat_btn: button::Style {
                background: Some(Background::Color(color!("#212121"))),
                ..chat_btn
            },
            profile_image_btn: button::Style {
                background: Some(Background::Color(color!("#D3D3D3"))),
                text_color: color!("#D3D3D3"),
                border: Border {
                    radius: Radius::new(100),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            connect_btn: button::Style {
                background: Some(Background::Color(color!("#ECECEC"))),
                text_color: color!("#1F1F1F"),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            input,
            input_container: container::Style {
                text_color: Some(color!("#D3D3D3")),
                background: Some(Background::Color(color!("#303030"))),
                border: Border {
                    radius: Radius::new(30),
                    ..Border::default()
                },
                ..container::Style::default()
            },
            rooms_container: container::Style {
                text_color: Some(color!("#D3D3D3")),
                background: Some(Background::Color(color!("#181818"))),
                ..container::Style::default()
            },
            users_container: container::Style {
                text_color: Some(color!("#D3D3D3")),
                background: Some(Background::Color(color!("#212121"))),
                ..container::Style::default()
            },
            message_container: container::Style {
                text_color: Some(color!("#D3D3D3")),
                background: Some(Background::Color(color!("#212121"))),
                ..container::Style::default()
            },
            message: container::Style {
                text_color: Some(Color::from_rgb(200.0, 44.0, 0.0)),
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                ..container::Style::default()
            },
        };

        let login_form = LoginForm {
            login_btn: button::Style {
                background: Some(Background::Color(color!("#D3D3D3"))),
                text_color: color!("#000000"),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                shadow: Shadow::default(),
            },
            form_container: container::Style {
                text_color: Some(color!("#D3D3D3")),
                background: Some(Background::Color(color!("#212121"))),
                border: Border {
                    radius: Radius::new(10),
                    ..Border::default()
                },
                ..container::Style::default()
            },
            background: container::Style {
                background: Some(Background::Color(color!("#181818"))),
                ..container::Style::default()
            },
            input,
        };

        let app = App {
            background: container::Style {
                text_color: Some(color!("#FFFFFF")),
                background: Some(Background::Color(color!("#181818"))),
                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            },
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
    pub send_btn_svg: svg::Handle,
    pub create_room_svg: svg::Handle,
    pub message_sent_svg: svg::Handle,
    pub message_received_svg: svg::Handle,
    pub message_read_svg: svg::Handle,
    pub send_btn: button::Style,
    pub connect_btn: button::Style,
    pub show_user_search_btn: button::Style,
    pub profile_image_svg: svg::Handle,
    pub profile_image_btn: button::Style,
    pub chat_btn: button::Style,
    pub active_chat_btn: button::Style,
    pub rooms_container: container::Style,
    pub users_container: container::Style,
    pub input: text_input::Style,
    pub input_container: container::Style,
    pub message_container: container::Style,
    pub message: container::Style,
}

pub struct LoginForm {
    pub background: container::Style,
    pub form_container: container::Style,
    pub login_btn: button::Style,
    pub input: text_input::Style,
}

pub struct App {
    pub background: container::Style,
}
