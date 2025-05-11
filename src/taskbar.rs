use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::Event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    println!("Connected to display. Root window: {}", screen.root);

    // Listen for window create/destroy events
    conn.change_window_attributes(screen.root, &ChangeWindowAttributesAux::new().event_mask(EventMask::SUBSTRUCTURE_NOTIFY))?;

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::CreateNotify(ev) => println!("Window created: {}", ev.window),
            Event::DestroyNotify(ev) => println!("Window destroyed: {}", ev.window),
            _ => {}
        }
    }
}
