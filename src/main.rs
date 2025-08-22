/*
    File: Cargo.toml
    Purpose: Defines the project's dependencies.
*/



/*
    File: src/main.rs
    Purpose: The main application logic with integrated RGB control.
*/

// Import Tokio's async versions of process and io utilities
use tokio::process::Command;
use tokio::io::{BufReader, AsyncBufReadExt};
use std::process::Stdio;

// Import the necessary components from the correct crates.
use rog_aura::{AuraEffect, AuraModeNum, AuraZone, Colour, Direction, Speed};
use rog_dbus::zbus_aura::AuraProxy;
// Import the zbus Connection struct and the ObjectManagerProxy for discovery
use zbus::{Connection, fdo::ObjectManagerProxy};


// By using `#[tokio::main]`, we set up the async runtime.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting StrixSense listener...");

    println!("--> Connecting to system D-Bus...");
    let connection = Connection::system().await?;

    println!("--> Finding Aura hardware service dynamically...");
    let aura = find_aura_proxy(&connection).await?;
    println!("--> Connection successful. Monitoring system journal for Bluetooth events.");

    let mut cmd = Command::new("journalctl")
        .arg("-u")
        .arg("bluetooth.service")
        .arg("-f")
        .arg("-n")
        .arg("0")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        if line.contains("ready") && line.contains(": fd(") {
            println!("[+] DEVICE CONNECTED (Detected via journal log)");
            println!("    -> Setting keyboard to STATIC GREEN");

            // --- LIGHTING CODE ---
            // Set the mode to Static.
            let green_effect = AuraEffect {
                mode: AuraModeNum::Static,
                zone: AuraZone::None,
                colour1: Colour { r: 0, g: 255, b: 0 },
                colour2: Colour { r: 0, g: 0, b: 0 },
                speed: Speed::Med,
                direction: Direction::Right,
            };
            aura.set_led_mode_data(green_effect).await?;
            println!("    -> Keyboard color set to STATIC GREEN!");


        } else if line.contains("device_disconnected") {
            println!("[-] DEVICE DISCONNECTED (Detected via journal log)");
            println!("    -> Setting keyboard to STATIC RED");

            // --- LIGHTING CODE ---
            // Set the mode to Static and the color to Red.
            let red_effect = AuraEffect {
                mode: AuraModeNum::Static,
                zone: AuraZone::None,
                colour1: Colour { r: 255, g: 0, b: 0 },
                colour2: Colour { r: 0, g: 0, b: 0 },
                speed: Speed::Med,
                direction: Direction::Right,
            };
            aura.set_led_mode_data(red_effect).await?;
            println!("    -> Keyboard color set to STATIC RED!");
        }
    }

    Ok(())
}

/// This function dynamically finds the correct D-Bus path for the Aura service.
async fn find_aura_proxy<'a>(connection: &'a Connection) -> Result<AuraProxy<'a>, Box<dyn std::error::Error>> {
    let manager = ObjectManagerProxy::new(connection, "xyz.ljones.Asusd", "/").await?;
    let objects = manager.get_managed_objects().await?;

    for (path, interfaces) in objects {
        if interfaces.contains_key("xyz.ljones.Aura") {
            println!("    -> Found Aura interface at path: {}", path);
            return Ok(AuraProxy::builder(connection).path(path)?.build().await?);
        }
    }

    Err("Could not find any object with the 'xyz.ljones.Aura' interface.".into())
}
