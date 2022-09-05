mod app;

use app::App;
use iced::{Settings, Application};
fn main() {
    App::run(
        Settings::default()
    ).unwrap()
}
