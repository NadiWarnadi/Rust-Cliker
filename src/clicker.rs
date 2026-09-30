use crate::config::{ClickType, ClickerConfig};
use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub fn start_clicker_thread(running: Arc<AtomicBool>, config: ClickerConfig) {
    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).expect("Gagal inisialisasi Enigo Engine");
        let interval = config.get_interval();

        let button = match config.click_type {
            ClickType::Left => Button::Left,
            ClickType::Right => Button::Right,
            ClickType::Middle => Button::Middle,
        };

        loop {
            if running.load(Ordering::SeqCst) {
                // Eksekusi simulasi klik
                let _ = enigo.button(button, Direction::Click);
                
                // Jeda sesuai target CPS
                thread::sleep(interval);
            } else {
                // Saat status OFF, tidurkan thread sebentar agar CPU Usage tetap mendekati 0%
                thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    });
}