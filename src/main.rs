use std::fs;
use std::process::Command;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    println!("Connected to X server. Root window ID: {}", screen.root);

    // Read start.conf lines (programs to run)
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".config/wire/start.conf");

    let contents = fs::read_to_string(config_path)?;
    for line in contents.lines() {
        let command = line.trim();
        if command.is_empty() || command.starts_with('#') {
            continue; // skip empty or commented lines
        }

        // Spawn each program (detach, don't wait)
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect(&format!("Failed to start command: {}", command));
    }

    // Now the main event loop (your X event handling)
    loop {
        let event = conn.wait_for_event()?;
        println!("Got event: {:?}", event);
        conn.flush()?;
    }
}
