
mod styles;
mod client;
mod battleship;
mod settings;



use iced::{Size};

use battleship::GameInfo;

fn main() -> iced::Result {
    
    let window_settings = iced::window::Settings {
        size: Size {
            width: 1080.0,
            height: 720.0,
        },
        resizable: true,
        exit_on_close_request: true,
        ..iced::window::Settings::default()
    };
    
    iced::application("Online Battle Ship", GameInfo::update, GameInfo::view)
        .window(window_settings)
        .run()
}