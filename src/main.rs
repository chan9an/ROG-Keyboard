
use tokio::time::{sleep, Duration};

// Import the necessary components from the correct crates.
use rog_aura::{AuraEffect, AuraModeNum, AuraZone, Colour, Direction, Speed};
use rog_dbus::zbus_aura::AuraProxy;
// Import the zbus Connection struct and the ObjectManagerProxy for discovery
use zbus::{Connection, fdo::ObjectManagerProxy};


// By using `#[tokio::main]`, we set up the async runtime.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting StrixSense light test...");

    println!("--> Connecting to system D-Bus...");
    let connection = Connection::system().await?;

    println!("--> Finding Aura hardware service dynamically...");
    let aura = find_aura_proxy(&connection).await?;
    println!("--> Connection successful. Now testing light control.");

    // --- TEST 1: SET TO GREEN ---
    println!("    -> Setting keyboard to STATIC GREEN");
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

    // Wait for 3 seconds to see the color change
    sleep(Duration::from_secs(3)).await;

    // --- TEST 2: SET TO RED ---
    println!("    -> Setting keyboard to STATIC RED");
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

    println!("--> Test complete.");

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
