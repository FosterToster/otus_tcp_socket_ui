use shtp::{SHTPClient, Result as SHTPResult, DeviceType};
use std::str::FromStr;

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

    fn try_connect(&self, host: String, port: u16) -> SHTPResult<SHTPClient> {
        
        let client = SHTPClient::new(host, port, DeviceType::SmartSocket);

        client.send_command("test".to_string(), vec![]).map(|_| client)
    }

    fn parse_value(&self) -> Option<(String, u16)> {
        let pair = self.value.split(':').collect::<Vec<&str>>();
        
        if pair.len() != 2 {
            return None
        }

        let host = pair[0];
        let port = pair[1];

        let host = host.split('.')
            .filter_map(|val| -> Option<String> {
                match u8::from_str(val) {
                    Ok(val) => Some(val.to_string()),
                    Err(_) => None
                }
            })
            .collect::<Vec<String>>();
        
        if host.len() != 4 {
            return None;
        }

        match u16::from_str(port) {
            Ok(val) => {
                Some((
                    host.iter().enumerate().fold(String::new(), |mut prev, (idx, cur)| {
                        if idx > 2 {
                            prev.push_str(cur.as_str());  
                        } else {
                            prev.push_str(format!("{}.", cur).as_str());
                        };
                        prev
                    }),
                    val
                ))
            },
            Err(_) => None
        }
    }

    pub fn update(&mut self, message:ConnectMessage) -> Option<SHTPClient> {
        match message {
            ConnectMessage::AddrMutated(val) => {
                self.error = None;
                self.value = val;
                None
            },
            ConnectMessage::TryConnect => {
                match  self.parse_value() {
                    Some((host, port)) => {
                        match self.try_connect(host, port) {
                            Ok(client) => {
                                self.error = None;
                                Some(client)
                            },
                            Err(e) => {
                                self.error = Some( format!("{}", e) );
                                None
                            }
                        }
                    },
                    None => {
                        self.error = Some("Неправильный формат ввода.".to_string());
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
                            text(format!("Ошибка: {}", err))
                        },
                        None => text("Введите адрес умной розетки"),
                    }.into(),
                    text_input("ip.ip.ip.ip:port", &self.value, |val| Message::Connect(ConnectMessage::AddrMutated(val)))
                        .padding(3)
                        .into(),
                    button("Подключить").on_press(Message::Connect(ConnectMessage::TryConnect)).into()
                ]
            ).spacing(5)
        )
            .center_x()
            .center_y()
            .into()
    }
}