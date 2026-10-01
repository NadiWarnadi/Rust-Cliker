use std::io::{self, Write};
use std::time::Duration;

#[allow(dead_code)] // Menghilangkan warning jika ada varian klik yang belum dipakai
#[derive(Debug, Clone, Copy)]
pub enum ClickType {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub struct ClickerConfig {
    pub interval: Duration,
    pub click_type: ClickType,
}

impl ClickerConfig {
    pub fn prompt_from_user() -> Self {
        println!("==================================================");
        println!("         RUST AUTOCLICKER CONFIGURATION           ");
        println!("==================================================");

        // 1. Pilih Satuan Waktu
        println!("> Pilih Satuan Waktu Jeda:");
        println!("  [1] Milidetik (ms)  --> Contoh: 100 ms = 0.1 detik");
        println!("  [2] Detik (s)       --> Contoh: 40 detik");
        println!("  [3] Menit (m)       --> Contoh: 2 menit");
        print!("  Pilihan (1/2/3) [Default: 1]: ");
        io::stdout().flush().unwrap();

        let mut unit_input = String::new();
        io::stdin().read_line(&mut unit_input).unwrap();
        let unit_choice = unit_input.trim();

        // 2. Input Angka Jeda Waktu
        print!("> Masukkan durasi jeda [Default: 100]: ");
        io::stdout().flush().unwrap();

        let mut duration_input = String::new();
        io::stdin().read_line(&mut duration_input).unwrap();
        let value: u64 = duration_input.trim().parse().unwrap_or(100);

        let interval = match unit_choice {
            "2" => Duration::from_secs(value),
            "3" => Duration::from_secs(value * 60),
            _   => Duration::from_millis(value),
        };

        // 3. Pilih Jenis Klik
        println!("\n> Pilih Jenis Klik:");
        println!("  [1] Klik Kiri (Left Click) - Default");
        println!("  [2] Klik Kanan (Right Click)");
        println!("  [3] Klik Tengah (Middle Click)");
        print!("  Pilihan Anda (1/2/3): ");
        io::stdout().flush().unwrap();

        let mut click_input = String::new();
        io::stdin().read_line(&mut click_input).unwrap();
        let click_type = match click_input.trim() {
            "2" => ClickType::Right,
            "3" => ClickType::Middle,
            _   => ClickType::Left,
        };

        println!("==================================================");

        Self {
            interval,
            click_type,
        }
    }
}