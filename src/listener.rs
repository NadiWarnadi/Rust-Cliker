use rdev::{listen, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn start_hotkey_listener(running: Arc<AtomicBool>, toggle_key: Key) {
    if let Err(error) = listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if key == toggle_key {
                let current_state = running.load(Ordering::SeqCst);
                let new_state = !current_state;
                running.store(new_state, Ordering::SeqCst);

                if new_state {
                    println!("[+] Autoclicker: AKTIF");
                } else {
                    println!("[-] Autoclicker: NONAKTIF");
                }
            }
        }
    }) {
        println!("Error pada Input Listener: {:?}", error);
    }
}