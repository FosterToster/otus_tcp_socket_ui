use shtp::{Result as SHTPResult, SHTPClient, SHTPResponse};

use iced::widget::{button, column, container, row, text};

use super::super::{ConnectedMessage, Message};

pub struct ConnectedView {
    client: SHTPClient,
    command_result: Option<String>,
}

impl ConnectedView {
    pub fn new(client: SHTPClient) -> Self {
        Self {
            client,
            command_result: None,
        }
    }

    pub fn update(&mut self, message: ConnectedMessage) {
        self.command_result = None;

        self.observe_result(match message {
            ConnectedMessage::Disconnect => {
                panic!("ConnectMessage::Disconnect should be handled on higher level.")
            }
            ConnectedMessage::GetConsumption => {
                self.client.send_command("consumption".to_string(), vec![])
            }
            ConnectedMessage::GetState => self.client.send_command("state".to_string(), vec![]),
            ConnectedMessage::SwitchOn => self
                .client
                .send_command("onoff".to_string(), vec!["on".to_string()]),
            ConnectedMessage::SwitchOff => self
                .client
                .send_command("onoff".to_string(), vec!["off".to_string()]),
        })
    }

    fn observe_result(&mut self, result: SHTPResult<SHTPResponse>) {
        match result {
            Ok(response) => match response.observe() {
                Ok(result) => self.command_result = Some(format!("Ответ >> {}", result)),
                Err(e) => self.command_result = Some(format!("Ответ >> Ошибка >> {}", e)),
            },
            Err(e) => self.command_result = Some(format!("Разрыв соединения: {}", e)),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        container(
            column(vec![
                row(vec![
                    text("Соединение установлено").into(),
                    button("Отключиться")
                        .on_press(Message::Connected(ConnectedMessage::Disconnect))
                        .into(),
                ])
                .into(),
                row(vec![
                    button("Текущее состояние")
                        .on_press(Message::Connected(ConnectedMessage::GetState))
                        .into(),
                    button("Включить")
                        .on_press(Message::Connected(ConnectedMessage::SwitchOn))
                        .into(),
                    button("Выключить")
                        .on_press(Message::Connected(ConnectedMessage::SwitchOff))
                        .into(),
                ])
                .spacing(10)
                .into(),
                button("Энергопотребление")
                    .on_press(Message::Connected(ConnectedMessage::GetConsumption))
                    .into(),
                {
                    if let Some(r) = &self.command_result {
                        text(r)
                    } else {
                        text("")
                    }
                }
                .into(),
            ])
            .spacing(5),
        )
        .center_x()
        .center_y()
        .into()
    }
}
