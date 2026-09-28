// src/config.rs
#[derive(Debug, Clone)]
pub struct ClickerConfig {
    pub window_title: String,
    pub interval_ms: u64,
    pub percent_x: f64, // 0.0 - 1.0 (0% - 100%)
    pub percent_y: f64, // 0.0 - 1.0 (0% - 100%)
}

#[derive(Debug, Clone)]
pub struct WorkerState {
    pub is_running: bool,
}
