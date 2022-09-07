mod app;

use iced::{Settings, Application, window};
fn main() {
    app::App::run(
        Settings {
            window: window::Settings {
                size: (600, 240),
                ..window::Settings::default()
            },
            default_font: Some(include_bytes!("../Roboto-Regular.ttf")),
            ..Settings::default()
        }
        
    ).unwrap()
}
