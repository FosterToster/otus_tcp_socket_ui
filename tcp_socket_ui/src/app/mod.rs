// mod app;
mod views;

// pub use app::{
//     App,
//     Message,
//     ConnectMessage,
//     ConnectedMessage
// };

use iced::{
    executor,
    widget::{column, text},
    Application, Command, Padding, Theme,
};

// use super::appstate::{AppState, Navigate};

use views::{ConnectView, ConnectedView};

pub enum App {
    Disconnected(ConnectView),
    Connected(ConnectedView),
}

#[derive(Debug, Clone)]
pub enum ConnectMessage {
    AddrMutated(String),
    TryConnect,
}

#[derive(Debug, Clone)]
pub enum ConnectedMessage {
    Disconnect,
    GetState,
    GetConsumption,
    SwitchOn,
    SwitchOff,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(ConnectMessage),
    Connected(ConnectedMessage),
}

impl Application for App {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::Disconnected(ConnectView {
                value: "".to_string(),
                error: None,
            }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Smart Socket App".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Self::Disconnected(state) => match message {
                Message::Connect(connect_message) => match state.update(connect_message) {
                    Some(shtp_client) => {
                        *self = App::Connected(ConnectedView::new(shtp_client));
                        Command::none()
                    }
                    _ => Command::none(),
                },
                _ => panic!("Bad state"),
            },
            Self::Connected(state) => match message {
                Message::Connected(connected_message) => {
                    if let ConnectedMessage::Disconnect = connected_message {
                        *self = App::Disconnected(ConnectView {
                            value: "".to_string(),
                            error: None,
                        });
                    } else {
                        state.update(connected_message)
                    }
                    Command::none()
                }
                _ => panic!("Bad state"),
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column(vec![
            text("Smart Socket App").size(40).into(),
            match self {
                Self::Disconnected(state) => state.view(),
                Self::Connected(state) => state.view(),
            },
        ])
        .spacing(10)
        .padding(Padding::from(10))
        .into()
    }
}
