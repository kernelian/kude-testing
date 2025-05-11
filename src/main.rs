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

    conn.flush()?;

    // Event loop
    loop {
        let event = conn.wait_for_event()?;
        println!("Got event: {:?}", event);
    }

    panel::launch_panel()
    
}
