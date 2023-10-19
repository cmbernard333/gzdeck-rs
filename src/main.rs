use gzdeck_rs::app;
use iced::Settings;
use iced::Application;

fn main() -> iced::Result {
    // NOTE here: the trait iced::Application has to be in scope
    app::GZDeckApplication::run(Settings::default())
}
