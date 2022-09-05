use iced::{
    Application,
    Theme,
    Settings,
    executor,
    Command,
    widget::{
        column, text, button
    }
};

pub struct App {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start
}

impl Application for App {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;
    
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self{},
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Smart Socket App".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column(
            vec![
                text("Hello, world!").into(),
                button("content").on_press(Message::Start).into()
            ]
        ).into()
    }
}