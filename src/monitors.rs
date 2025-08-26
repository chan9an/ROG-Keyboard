use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

// Import the light control functions from our new `lights` module.
use crate::lights::{set_keyboard_breathe, set_keyboard_static};

/// Monitors `bluetoothctl` output for connection events.
pub fn monitor_bluetooth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("bluetoothctl")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().expect("Failed to capture stdout from bluetoothctl");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let text = line?;
        if text.contains("Connected: yes") {
            println!("[+] BLUETOOTH DEVICE CONNECTED");
            // Breathe a vibrant lime green, then return to default indigo
            set_keyboard_breathe("32CD32")?; // Lime Green
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("4B0082")?; // Indigo
        } else if text.contains("Connected: no") {
            println!("[-] BLUETOOTH DEVICE DISCONNECTED");
            // Breathe a deep red, then return to default indigo
            set_keyboard_breathe("8B0000")?; // Deep Red
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("4B0082")?; // Indigo
        }
    }
    Ok(())
}

/// Monitors `nmcli monitor` output for WiFi connection events.
pub fn monitor_wifi() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("nmcli")
        .arg("monitor")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().expect("Failed to capture stdout from nmcli");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let text = line?;
        if text.contains("' is now the primary connection") {
            println!("[+] WIFI CONNECTED");
            // Breathe a cool sky blue, then return to default indigo
            set_keyboard_breathe("87CEEB")?; // Sky Blue
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("4B0082")?; // Indigo
        } else if text.contains("There's no primary connection") {
            println!("[-] WIFI DISCONNECTED");
            // Breathe a warning gold color, then return to default indigo
            set_keyboard_breathe("FFD700")?; // Gold
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("4B0082")?; // Indigo
        }
    }
    Ok(())
}
