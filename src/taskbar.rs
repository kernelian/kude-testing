use chrono::Local;
use std::{error::Error, thread, time::Duration};
use x11rb::{
    connection::Connection,
    protocol::xproto::*,
    rust_connection::RustConnection,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Taskbar running!"); // Ensuring
    let (conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let width = 400;
    let height = 24;

    let win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        win,
        screen.root,
        0,
        (screen.height_in_pixels - height) as i16,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;
    conn.map_window(win)?;
    conn.flush()?;

    // Create graphics context for drawing text
    let gc = conn.generate_id()?;
    conn.create_gc(gc, win, &CreateGCAux::new().foreground(screen.black_pixel))?;

    // Start the clock thread
    let conn_clone = conn.clone();
    thread::spawn(move || loop {
        let time = Local::now().format("%H:%M:%S").to_string();
        let _ = conn_clone.poly_text_8(win, gc, 10, 16, time.as_bytes());
        let _ = conn_clone.flush();
        thread::sleep(Duration::from_secs(1));
    });

    // Event loop
    loop {
        let _ = conn.wait_for_event()?;
    }
}
