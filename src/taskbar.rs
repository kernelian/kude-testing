use chrono::Local;
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use x11rb::{
    connection::Connection,
    protocol::xproto::*,
    rust_connection::RustConnection,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the X server
    let (mut conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let width = 400;
    let height = 24;

    // Create the window
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
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;
    conn.map_window(win)?;
    conn.flush()?;

    // Create a font
    let font = conn.generate_id()?;
    match conn.open_font(font, b"-*-dejavu-sans-medium-r-normal-*-50-*-*-*-*-*-*-*") {
        Ok(_) => println!("Font loaded successfully"),
        Err(e) => eprintln!("Error loading font: {}", e),
    }

    // Create GC
    let gc = conn.generate_id()?;
    conn.create_gc(
        gc,
        win,
        &CreateGCAux::new()
            .foreground(0) // Black text
            .background(screen.white_pixel)
            .font(font),
    )?;

    // Draw static text on the taskbar
    match conn.poly_text8(win, gc, 10, 16, b"Hello, World!") {
        Ok(_) => println!("Text drawn successfully"),
        Err(e) => eprintln!("Error drawing text: {}", e),
    }
    
    // Make sure to flush the drawing commands
    conn.flush()?;

    // Wrap connection in Arc<Mutex<...>> to share safely between threads
    let shared_conn = Arc::new(Mutex::new(conn));

    // Clone for the clock thread
    let clock_conn = Arc::clone(&shared_conn);

    // Start a thread for updating the clock every second
    thread::spawn(move || loop {
        let time = Local::now().format("%H:%M:%S").to_string();

        let conn = clock_conn.lock().unwrap();
        let _ = conn.poly_text8(win, gc, 10, 16, time.as_bytes());
        let _ = conn.flush();

        thread::sleep(Duration::from_secs(1));
    });

    // Main event loop
    let main_conn = Arc::clone(&shared_conn);
    loop {
        let conn = main_conn.lock().unwrap();
        let _ = conn.wait_for_event()?;
    }
}
