use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the X server
    let (conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;

    // Get the screen
    let screen = &conn.setup().roots[screen_num];

    println!("Connected to X server. Root window ID: {}", screen.root);

    // Window dimensions
    let width = screen.width_in_pixels;
    let height = 24;

    // Create the taskbar window
    let win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        win,
        screen.root,
        0,  // x position
        (screen.height_in_pixels - height) as i16, // y position
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;

    // Make sure the window is shown
    conn.map_window(win)?;
    conn.flush()?;  // Flush to ensure the window is actually rendered

    println!("Taskbar window created and mapped.");

    // Event loop
    loop {
        let event = conn.wait_for_event()?;
        println!("Got event: {:?}", event);

        // Flush after each event to keep things smooth
        conn.flush()?;
    }
}
