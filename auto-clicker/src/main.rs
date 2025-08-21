use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};
use std::{thread, time::Duration};
use std::time::SystemTime;

fn get_timestamp() -> String {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("[{}]", time)
}

fn main() {
    let device_state = DeviceState::new();
    let mut enigo = Enigo::new();
    
    println!("{} Auto-clicker started! Press 'c' to stop (works from any window).", get_timestamp());
    
    loop {
        // Check if 'c' is pressed
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::C) {
            println!("{} Stopping auto-clicker...", get_timestamp());
            break;
        }

        // Perform right click
        enigo.mouse_click(MouseButton::Right);
        
        // Small delay to prevent too rapid clicking
        thread::sleep(Duration::from_millis(50));
    }
}
