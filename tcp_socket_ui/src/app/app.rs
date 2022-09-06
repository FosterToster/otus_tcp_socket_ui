use iced::{
    Application,
    Sandbox,
    Command,
    Theme,
    Settings,
    executor,
    Padding,
    widget::{
        column, text, button
    }
};

// use super::appstate::{AppState, Navigate};

use super::views::{ConnectView, ConnectedView};

pub enum App {
    Disconnected(ConnectView),
    Connected(ConnectedView),
}

#[derive(Debug, Clone)]
pub enum ConnectMessage {
    AddrMutated(String),
    TryConnect
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(ConnectMessage)
}

impl Application for App {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;
    
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::Disconnected(ConnectView{value: "".to_string(), error: None}),
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Smart Socket App".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Self::Disconnected(state) => {
                match message {
                    Message::Connect(connect_message) => {
                        match state.update(connect_message) {
                            Some(shtp_client) => {
                                *self = App::Connected(ConnectedView::new(shtp_client));
                                Command::none()
                            },
                            _ => Command::none()
                        }
                    },
                    _ => panic!("Bad state")
                }
            },
            _ => Command::none()
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column(
            vec![
                text("Панель управления умной розеткой").size(20).into(),
                match self {
                    Self::Disconnected(state) => state.view(),
                    _ => text("Вы великолепны").into()
                    
                }
            ]
        )
            .padding(Padding::from(10))
            .into()
    }
}