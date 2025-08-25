
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// Main function that spawns listener threads.
fn main() {
    println!("Starting StrixSense Listener...");

    // Spawn a new thread to handle Bluetooth monitoring.
    let bluetooth_handle = thread::spawn(|| {
        if let Err(e) = monitor_bluetooth() {
            eprintln!("Bluetooth monitor failed: {}", e);
        }
    });

    // Spawn a new thread to handle WiFi monitoring.
    let wifi_handle = thread::spawn(|| {
        if let Err(e) = monitor_wifi() {
            eprintln!("WiFi monitor failed: {}", e);
        }
    });

    println!("--> Monitoring for Bluetooth and WiFi events...");

    // Wait for the threads to complete (they will run forever).
    bluetooth_handle.join().unwrap();
    wifi_handle.join().unwrap();
}

/// Monitors `bluetoothctl` output for connection events.
fn monitor_bluetooth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("bluetoothctl")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().expect("Failed to capture stdout from bluetoothctl");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let text = line?;
        if text.contains("Connected: yes") {
            println!("[+] BLUETOOTH DEVICE CONNECTED");
            // Breathe green for 3 seconds, then return to default blue
            set_keyboard_breathe("00ff00")?; // Green
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("0000ff")?; // Blue
        } else if text.contains("Connected: no") {
            println!("[-] BLUETOOTH DEVICE DISCONNECTED");
            // Breathe red for 3 seconds, then return to default blue
            set_keyboard_breathe("ff0000")?; // Red
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("0000ff")?; // Blue
        }
    }
    Ok(())
}

/// Monitors `nmcli monitor` output for WiFi connection events.
fn monitor_wifi() -> Result<(), Box<dyn std::error::Error>> {
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
            // Breathe a bright cyan color, then return to default blue
            set_keyboard_breathe("00FFFF")?; // Cyan
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("0000ff")?; // Blue
        } else if text.contains("There's no primary connection") {
            println!("[-] WIFI DISCONNECTED");
            // Breathe a warning orange color, then return to default blue
            set_keyboard_breathe("FFA500")?; // Orange
            thread::sleep(Duration::from_secs(3));
            set_keyboard_static("0000ff")?; // Blue
        }
    }
    Ok(())
}

/// A helper function to set a BREATHE effect by calling the `asusctl` command.
fn set_keyboard_breathe(color_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("    -> Setting keyboard to BREATHE with color #{}", color_hex);

    // Execute `asusctl aura breathe -c <color> -C 000000 -s high`
    // This will breathe from the specified color to black at a high speed.
    let output = Command::new("asusctl")
        .arg("aura")
        .arg("breathe")
        .arg("-c")
        .arg(color_hex)
        .arg("-C")
        .arg("000000") // Second color is black
        .arg("-s")
        .arg("high")   // Speed is high
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("    -> Error from asusctl: {}", error_message);
    } else {
        println!("    -> Breathe effect activated!");
    }

    Ok(())
}


/// A helper function to set a STATIC color by calling the `asusctl` command.
fn set_keyboard_static(color_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("    -> Setting keyboard to STATIC with color #{}", color_hex);

    let output = Command::new("asusctl")
        .arg("aura")
        .arg("static")
        .arg("-c")
        .arg(color_hex)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("    -> Error from asusctl: {}", error_message);
    } else {
        println!("    -> Static color set!");
    }

    Ok(())
}
