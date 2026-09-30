mod config;
mod clicker;
mod listener;

use config::{ClickType, ClickerConfig};
use rdev::Key;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    // 1. Inisialisasi Konfigurasi (Contoh: 10 CPS, Klik Kiri)
    let config = ClickerConfig::new(10, ClickType::Left);

    // 2. Shared State untuk Status Aktif/Nonaktif (Thread-Safe)
    let running = Arc::new(AtomicBool::new(false));

    println!("==================================================");
    println!("       RUST AUTOCLICKER - SYSTEM ARCHITECTURE    ");
    println!("==================================================");
    println!(" Configuration : {} CPS | Type: {:?}", config.cps, config.click_type);
    println!(" Hotkey        : Press [F8] to Start / Stop");
    println!(" Quit          : Press [Ctrl + C] in terminal");
    println!("==================================================");

    // 3. Jalankan Worker Thread untuk Simulasi Klik
    clicker::start_clicker_thread(Arc::clone(&running), config);

    // 4. Jalankan Listener Thread di Main Thread (Blocking)
    listener::start_hotkey_listener(Arc::clone(&running), Key::F8);
}