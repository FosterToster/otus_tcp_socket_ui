mod app;

use iced::{Settings, Application, window};
fn main() {
    app::App::run(
        Settings {
            window: window::Settings {
                size: (320, 240),
                ..window::Settings::default()
            },
            ..Settings::default()
        }
        
    ).unwrap()
}
