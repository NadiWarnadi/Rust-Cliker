// src/main.rs
#![windows_subsystem = "windows"]

mod config;
mod app;
mod worker;

use iced::{Application, Settings, Size};
use app::ClickerApp;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = Size::new(400.0, 420.0);
    settings.window.resizable = false;
    
    ClickerApp::run(settings)
}
