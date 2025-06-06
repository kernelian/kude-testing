use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the X server
    let (conn, screen_num): (RustConnection, usize) = RustConnection::connect(None)?;

    // Get the screen
    let screen = &conn.setup().roots[screen_num];

    println!("Connected to X server. Root window ID: {}", screen.root);

    // Events
    loop {
        let event = conn.wait_for_event()?;
        println!("Got event: {:?}", event);

        // Flush after each event to keep things smooth
        conn.flush()?;
    }
}
