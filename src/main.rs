use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the X server
    let (conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;

    // Get the screen
    let screen = &conn.setup().roots[screen_num];

    println!("Connected to X server. Root window ID: {}", screen.root);

    // Set up to receive window manager events
    conn.change_window_attributes(
        screen.root,
        &ChangeWindowAttributesAux::new().event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::STRUCTURE_NOTIFY,
        ),
    )?;

    // Create taskbar window
    let width = screen.width_in_pixels;
    let height = 24;
    let win = conn.generate_id()?;

    let aux = CreateWindowAux::new()
        .background_pixel(screen.black_pixel)
        .event_mask(EventMask::EXPOSURE)
        .override_redirect(1);

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
        &aux,
    )?;

    conn.map_window(win)?;
    conn.flush()?;

    // Event loop
    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(_) => {
                // Redraw logic (no-op for now)
                println!("Expose event received");
            }
            _ => {}
        }
    }
}
