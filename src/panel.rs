use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Orientation, Box as GtkBox};
use glib::{timeout_add_seconds_local, Continue};
use std::cell::RefCell;
use std::rc::Rc;
use chrono::Local;

fn main() {
    let app = Application::builder()
        .application_id("com.example.GtkPanel")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Create a top-level window with no decorations (like a panel)
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Panel")
        .default_width(800) // adjust based on screen
        .default_height(24)
        .decorated(false)
        .resizable(false)
        .build();

    window.set_keep_above(true); // Always on top
    window.move_(0, 0); // Top of screen

    let container = GtkBox::new(Orientation::Horizontal, 5);
    let clock_label = Label::new(None);
    container.pack_start(&clock_label, true, true, 10);
    window.add(&container);

    let clock = Rc::new(RefCell::new(clock_label));
    update_clock(Rc::clone(&clock));
    timeout_add_seconds_local(1, move || {
        update_clock(Rc::clone(&clock));
        Continue(true)
    });

    window.show_all();
}

fn update_clock(clock_label: Rc<RefCell<Label>>) {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    clock_label.borrow().set_text(&time_str);
}
