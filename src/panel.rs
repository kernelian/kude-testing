fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Panel")
        .default_width(800)
        .default_height(24)
        .decorated(false)
        .resizable(false)
        .build();

    window.set_keep_above(true);
    window.move_(0, 0);

    let container = GtkBox::new(Orientation::Horizontal, 5);
    let clock_label = Label::new(None);
    container.pack_start(&clock_label, true, true, 10);
    window.add(&container);

    // 🔧 Apply CSS: white background, black text
    let css = b"* { background-color: white; color: black; }";
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css).expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Failed to get screen"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let clock = Rc::new(RefCell::new(clock_label));
    update_clock(Rc::clone(&clock));
    timeout_add_seconds_local(1, move || {
        update_clock(Rc::clone(&clock));
        Continue(true)
    });

    window.show_all();
}
