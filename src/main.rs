use std::{error::Error, thread, time::Duration};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

use cairo::{Context, Format, ImageSurface};
use chrono::Local;

fn draw_clock(conn: &RustConnection, win: Window, gc: Gcontext, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)?;
    let cr = Context::new(&surface)?;

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint()?;

    let time_text = Local::now().format("%H:%M:%S").to_string();
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.set_font_size(18.0);

    let extents = cr.text_extents(&time_text)?;
    let x = (width as f64 - extents.width()) / 2.0 - extents.x_bearing();
    let y = (height as f64 - extents.height()) / 2.0 - extents.y_bearing();
    cr.move_to(x, y);
    cr.show_text(&time_text)?;
    cr.stroke()?;

    surface.flush();

    let data = surface.data()?;

    conn.put_image(
        ImageFormat::Z_PIXMAP,
        win,
        gc,
        width,
        height,
        0,
        0,
        0,
        24,
        &data,
    )?;

    conn.flush()?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let width = screen.width_in_pixels;
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

    // Create your own GC
    let gc = conn.generate_id()?;
    conn.create_gc(gc, win, &CreateGCAux::new())?;

    println!("Panel window created.");

    conn.change_window_attributes(
        win,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::EXPOSURE | EventMask::BUTTON_PRESS),
    )?;

    draw_clock(&conn, win, gc, width, height)?;

    loop {
        while let Ok(event) = conn.poll_for_event() {
            match event {
                Some(event) => match event {
                    x11rb::protocol::Event::Expose(_) => {
                        draw_clock(&conn, win, gc, width, height)?;
                    }
                    x11rb::protocol::Event::ButtonPress(_) => {
                        println!("Panel clicked!");
                    }
                    _ => {}
                },
                None => break,
            }
        }

        draw_clock(&conn, win, gc, width, height)?;

        thread::sleep(Duration::from_secs(1));
    }
}
