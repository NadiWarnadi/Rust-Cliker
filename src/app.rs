// src/app.rs
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Length, Application, Command, Theme};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::config::{ClickerConfig, WorkerState};
use crate::worker::start_worker_threads;

pub struct ClickerApp {
    pub shared_config: Arc<Mutex<ClickerConfig>>,
    pub shared_state: Arc<Mutex<WorkerState>>,
    pub window_title_input: String,
    pub interval_input: String,
    pub percent_x_input: String,
    pub percent_y_input: String,
    pub status_message: String,
}

#[derive(Debug, Clone)]
pub enum UiMessage {
    WindowTitleChanged(String),
    IntervalChanged(String),
    PercentXChanged(String),
    PercentYChanged(String),
    ToggleClicker,
    Tick,
}

impl Application for ClickerApp {
    type Executor = iced::executor::Default;
    type Message = UiMessage;
    type Theme = Theme;
    type Flags = ();

    // SOLUSI ERROR 1: Konstruktor Application Iced yang benar
    fn new(_flags: ()) -> (Self, Command<UiMessage>) {
        let shared_config = Arc::new(Mutex::new(ClickerConfig {
            window_title: String::from("Untitled - Notepad"),
            interval_ms: 200,
            percent_x: 0.5, // Default 50% (Tengah layar jendela)
            percent_y: 0.5,
        }));

        let shared_state = Arc::new(Mutex::new(WorkerState { is_running: false }));

        // Jalankan background worker Win32 API saat aplikasi pertama kali dibuka
        start_worker_threads(shared_config.clone(), shared_state.clone());

        (
            Self {
                shared_config,
                shared_state,
                window_title_input: String::from("Untitled - Notepad"),
                interval_input: String::from("200"),
                percent_x_input: String::from("50"),
                percent_y_input: String::from("50"),
                status_message: String::from("Status: Nonaktif (F8 untuk Toggle)"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Background Clicker Pro")
    }

    fn update(&mut self, message: UiMessage) -> Command<UiMessage> {
        match message {
            UiMessage::WindowTitleChanged(val) => {
                self.window_title_input = val.clone();
                self.shared_config.lock().unwrap().window_title = val;
            }
            UiMessage::IntervalChanged(val) => {
                if val.parse::<u64>().is_ok() || val.is_empty() {
                    self.interval_input = val.clone();
                    if let Ok(ms) = val.parse::<u64>() {
                        self.shared_config.lock().unwrap().interval_ms = ms;
                    }
                }
            }
            UiMessage::PercentXChanged(val) => {
                if val.parse::<f64>().is_ok() || val.is_empty() {
                    self.percent_x_input = val.clone();
                    if let Ok(pct) = val.parse::<f64>() {
                        // Ubah input persen 0-100 menjadi rasio skala 0.0 - 1.0
                        self.shared_config.lock().unwrap().percent_x = pct / 100.0;
                    }
                }
            }
            UiMessage::PercentYChanged(val) => {
                if val.parse::<f64>().is_ok() || val.is_empty() {
                    self.percent_y_input = val.clone();
                    if let Ok(pct) = val.parse::<f64>() {
                        self.shared_config.lock().unwrap().percent_y = pct / 100.0;
                    }
                }
            }
            UiMessage::ToggleClicker => {
                let mut st = self.shared_state.lock().unwrap();
                st.is_running = !st.is_running;
            }
            UiMessage::Tick => {
                let is_running = self.shared_state.lock().unwrap().is_running;
                if is_running {
                    self.status_message = String::from("Status: AKTIF (Mengklik di Latar Belakang)");
                } else {
                    self.status_message = String::from("Status: Nonaktif (Tekan F8)");
                }
            }
        }
        Command::none()
    }

    #[allow(mismatched_lifetime_syntaxes)]
    fn subscription<'a>(&'a self) -> iced::Subscription<UiMessage> {
        iced::time::every(Duration::from_millis(100)).map(|_| UiMessage::Tick)
    }

    // Baris 121



    
    // Tambahkan <'_, UiMessage> agar lifetime data dari &self tersinkron dengan Element GUI
    fn view(&self) -> Element<'_, UiMessage> {
        let is_running = self.shared_state.lock().unwrap().is_running;

        column![
            text("Rust Background Clicker (Persentase)").size(20),
            
            column![
                text("Judul Jendela Target (Exact):"),
                text_input("Untitled - Notepad", &self.window_title_input).on_input(UiMessage::WindowTitleChanged).padding(6)
            ].spacing(3),

            column![
                text("Jeda Klik (ms):"),
                text_input("200", &self.interval_input).on_input(UiMessage::IntervalChanged).padding(6)
            ].spacing(3),

            column![
                text("Posisi X Dinamis (Persen Jendela 0-100%):"),
                text_input("50", &self.percent_x_input).on_input(UiMessage::PercentXChanged).padding(6)
            ].spacing(3),

            column![
                text("Posisi Y Dinamis (Persen Jendela 0-100%):"),
                text_input("50", &self.percent_y_input).on_input(UiMessage::PercentYChanged).padding(6)
            ].spacing(3),

            button(if is_running { "STOP (F8)" } else { "START (F8)" })
                .padding(10)
                .width(Length::Fill)
                .on_press(UiMessage::ToggleClicker),

            text(&self.status_message).size(14),
        ]
        .spacing(12)
        .padding(15)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
