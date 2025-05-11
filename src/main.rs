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


    use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::Bool32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the X server
    let (conn, screen_num) = RustConnection::connect(None)?;

    // Get the screen and root window
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    // Define the taskbar's width and height
    let width = screen.width_in_pixels;
    let height = 24;

    // Generate a new window ID for the taskbar
    let win = conn.generate_id()?;

    // Create the taskbar window
    conn.create_window(
        screen.root_depth,
        win,
        root,
        0, // x position
        (screen.height_in_pixels - height) as i16, // y position (bottom of screen)
        width, // width
        height, // height
        0, // border width
        WindowClass::INPUT_OUTPUT,
        0, // visual
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;

    // Set override_redirect so window manager ignores the taskbar
    conn.change_window_attributes(win, &ChangeWindowAttributesAux::new().override_redirect(Bool32::from(true)))?;

    // Map (show) the window
    conn.map_window(win)?;
    conn.flush()?;

    // Event loop to keep the window open
    loop {
        conn.wait_for_event()?;
    }
}

}

    
}
