mod config;
mod clicker;
mod listener;

use config::ClickerConfig;
use rdev::Key;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    // 1. Minta input interaktif saat .exe pertama kali dibuka
    let config = ClickerConfig::prompt_from_user();

    // 2. State untuk toggle ON/OFF
    let running = Arc::new(AtomicBool::new(false));

    println!("\n[STATUS] Konfigurasi Berhasil Diatur!");
    println!(" -> Jeda Klik : {:?}", config.interval);
    println!(" -> Tipe Klik : {:?}", config.click_type);
    println!("\n[HOTKEY]");
    println!(" -> Tekan [F8] untuk START / STOP Autoclicker");
    println!(" -> Tekan [Ctrl + C] untuk keluar dari aplikasi");
    println!("==================================================\n");

    // 3. Jalankan Worker Thread untuk melakukan klik
    clicker::start_clicker_thread(Arc::clone(&running), config);

    // 4. Jalankan Listener Thread untuk menangkap tombol F8
    listener::start_hotkey_listener(Arc::clone(&running), Key::F8);
}