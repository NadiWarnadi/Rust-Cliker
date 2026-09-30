use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum ClickType {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub struct ClickerConfig {
    pub cps: u64,             // Clicks Per Second
    pub click_type: ClickType,
}

impl ClickerConfig {
    pub fn new(cps: u64, click_type: ClickType) -> Self {
        Self { cps, click_type }
    }

    // Menghitung interval jeda antar-klik berdasarkan CPS
    pub fn get_interval(&self) -> Duration {
        if self.cps == 0 {
            Duration::from_millis(1000)
        } else {
            Duration::from_micros(1_000_000 / self.cps)
        }
    }
}