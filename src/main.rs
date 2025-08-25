
/*
    File: src/main.rs
    Purpose: The complete program that listens for Bluetooth events and
             controls the keyboard lights by executing system commands.
*/

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

/// Main function that sets up the listener and controls the lights.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting StrixSense Listener...");
    println!("--> Monitoring bluetoothctl for events.");

    // Spawn the `bluetoothctl` command, which gives us clear, reliable output.
    let mut cmd = Command::new("bluetoothctl")
        .stdout(Stdio::piped())
        .spawn()?;

    // Get a handle to the command's output stream.
    let stdout = cmd.stdout.take().expect("Failed to capture stdout from bluetoothctl");
    let reader = BufReader::new(stdout);

    // Loop forever, reading each new line from the log.
    for line in reader.lines() {
        let text = line?;
        
        // Check for the definitive keywords from the bluetoothctl output.
        if text.contains("Connected: yes") {
            println!("[+] DEVICE CONNECTED");
            // Pulse green for 1 second as a notification
            set_keyboard_pulse("00ff00")?;
            sleep(Duration::from_secs(1));
            // Then return to a default static blue color
            set_keyboard_static("0000ff")?;
        } else if text.contains("Connected: no") {
            println!("[-] DEVICE DISCONNECTED");
            // Pulse red for 1 second as a notification
            set_keyboard_pulse("ff0000")?;
            sleep(Duration::from_secs(1));
            // Then return to a default static blue color
            set_keyboard_static("002FFF")?;
        }
    }

    Ok(())
}

/// A helper function to set a PULSE effect by calling the `asusctl` command.
fn set_keyboard_pulse(color_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("    -> Setting keyboard to PULSE with color #{}", color_hex);

    let output = Command::new("asusctl")
        .arg("aura")
        .arg("pulse")
        .arg("-c")
        .arg(color_hex)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("    -> Error from asusctl: {}", error_message);
    } else {
        println!("    -> Pulse effect activated!");
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
