
use std::thread;

// Declare the new modules we've created. This tells Rust to look for
// `lights.rs` and `monitors.rs` and include them in the build.
mod lights;
mod monitors;

/// Main function that spawns listener threads.
fn main() {
    println!("Starting StrixSense Listener...");

    // Spawn a new thread to handle Bluetooth monitoring.
    // We now call the function from its dedicated module.
    let bluetooth_handle = thread::spawn(|| {
        if let Err(e) = monitors::monitor_bluetooth() {
            eprintln!("[ERROR] Bluetooth monitor failed: {}", e);
        }
    });

    // Spawn a new thread to handle WiFi monitoring.
    // We now call the function from its dedicated module.
    let wifi_handle = thread::spawn(|| {
        if let Err(e) = monitors::monitor_wifi() {
            eprintln!("[ERROR] WiFi monitor failed: {}", e);
        }
    });

    println!("--> Monitoring for Bluetooth and WiFi events...");

    // Wait for the threads to complete (they will run forever).
    bluetooth_handle.join().unwrap();
    wifi_handle.join().unwrap();
}
