
use std::process::Command;

/// A helper function to set a BREATHE effect by calling the `asusctl` command.
pub fn set_keyboard_breathe(color_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("    -> Setting keyboard to BREATHE with color #{}", color_hex);

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
pub fn set_keyboard_static(color_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
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
