use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use std::{error::Error, thread};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Taskbar running!");

    // Connect to the X server
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    println!("Connected to display. Root window: {}", screen.root);

    // Window width and height
    let width = 400;
    let height = 24;

    // Create the window ID
    let win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        win,
        screen.root,
        0,
        (screen.height_in_pixels - height) as i16, // Position the taskbar at the bottom
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;

    // Map the window to the screen
    conn.map_window(win)?;
    conn.flush()?;

    // Create graphics context (GC)
    let gc = conn.generate_id()?;
    conn.create_gc(gc, win, &CreateGCAux::new().foreground(screen.black_pixel))?;

    // Load a default font
    let font = conn.generate_id()?;
    conn.open_font(font, b"-misc-fixed-*-*-*-*-13-*-*-*-*-*-*-*")?;

    // Set up the GC to use the font
    conn.create_gc(
        gc,
        win,
        &CreateGCAux::new()
            .foreground(0) // Black text
            .background(screen.white_pixel)
            .font(font),
    )?;

    // Text drawing loop (will show "Hello, World!" every second)
    loop {
        // Draw the time or any other text you want
        let time = chrono::Local::now().format("%H:%M:%S").to_string();
        let _ = conn.poly_text8(win, gc, 10, 16, time.as_bytes()); // Draw the time
        let _ = conn.flush(); // Flush to apply the changes

        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
    }

    // Wait for events to keep the window responsive
    loop {
        let _ = conn.wait_for_event()?;
    }
}
