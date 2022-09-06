use shtp::{SHTPClient, Result as SHTPResult, DeviceType};


use iced::widget::{
    button,
    text,
    text_input,
    container,
    column
};


use super::super::{Message, ConnectMessage};
pub struct ConnectView{
    pub value: String,
    pub error: Option<String>
}

impl ConnectView {

    fn try_connect(&self) -> SHTPResult<SHTPClient> {
        
        let client = SHTPClient::new("127.0.0.1".to_string(), 6411, DeviceType::SmartSocket);

        client.send_command("test".to_string(), vec![]).map(|_| client)
    }

    pub fn update(&mut self, message:ConnectMessage) -> Option<SHTPClient> {
        match message {
            ConnectMessage::AddrMutated(val) => {
                self.value = val;
                None
            },
            ConnectMessage::TryConnect => {
                match self.try_connect() {
                    Ok(client) => Some(client),
                    Err(e) => {
                        self.error = Some( format!("{}", e) );
                        None
                    }
                }
            }
        }    
    }

    pub fn view(&self) -> iced::Element<Message>{
        container(
            column(
                vec![
                    match &self.error{
                        Some(err) => {
                            text(format!("Ошибка подключения: {}", err))
                        },
                        None => text("Введите адрес умной розетки"),
                    }.into(),
                    text_input("ip.ip.ip.ip:port", &self.value, |val| Message::Connect(ConnectMessage::AddrMutated(val))).into(),
                    button("Подключить").on_press(Message::Connect(ConnectMessage::TryConnect)).into()
                ]
            ).spacing(5)
        )
            .center_x()
            .center_y()
            .into()
    }
}