// src/worker.rs
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::config::{ClickerConfig, WorkerState};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, RECT};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, PostMessageW, GetClientRect, WM_LBUTTONDOWN, WM_LBUTTONUP};

fn to_wstring(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn start_worker_threads(
    shared_config: Arc<Mutex<ClickerConfig>>,
    shared_state: Arc<Mutex<WorkerState>>,
) {
    let config_clone = shared_config.clone();
    let state_clone = shared_state.clone();
    
    tokio::spawn(async move {
        let mut rect = RECT::default();
        loop {
            let (run, title, delay, p_x, p_y) = {
                let cfg = config_clone.lock().unwrap();
                let st = state_clone.lock().unwrap();
                (st.is_running, cfg.window_title.clone(), cfg.interval_ms, cfg.percent_x, cfg.percent_y)
            };

            if run {
                unsafe {
                    let title_wide = to_wstring(&title);
                    let hwnd: HWND = FindWindowW(None, PCWSTR(title_wide.as_ptr()));

                    // SOLUSI ERROR 2: Pengecekan HWND kosong/invalid yang benar pada win32 API
                    if hwnd.0 != 0 {
                        if GetClientRect(hwnd, &mut rect).is_ok() {
                            let width = rect.right - rect.left;
                            let height = rect.bottom - rect.top;

                            let dynamic_x = (width as f64 * p_x) as i32;
                            let dynamic_y = (height as f64 * p_y) as i32;

                            let l_param = LPARAM(((dynamic_y << 16) | (dynamic_x & 0xFFFF)) as isize);
                            
                            let _ = PostMessageW(hwnd, WM_LBUTTONDOWN, WPARAM(1), l_param);
                            tokio::time::sleep(Duration::from_millis(5)).await;
                            let _ = PostMessageW(hwnd, WM_LBUTTONUP, WPARAM(0), l_param);
                        }
                    }
                }
                tokio::time::sleep(Duration::from_millis(delay)).await;
            } else {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    });

    let state_f8 = shared_state;
    tokio::spawn(async move {
        let _ = rdev::listen(move |event| {
            if let rdev::EventType::KeyPress(rdev::Key::F8) = event.event_type {
                let mut st = state_f8.lock().unwrap();
                st.is_running = !st.is_running;
            }
        });
    });
}
