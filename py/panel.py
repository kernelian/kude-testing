import gi
gi.require_version("Gtk", "3.0")
gi.require_version("Wnck", "3.0")
from gi.repository import Gtk, Gdk, Wnck, GLib, GdkPixbuf
import time

class Panel(Gtk.Window):
    def __init__(self):
        super().__init__(type=Gtk.WindowType.TOPLEVEL)

        self.set_decorated(False)
        self.set_keep_above(True)
        self.set_type_hint(Gdk.WindowTypeHint.DOCK)

        self.box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        self.box.set_margin_start(10)
        self.box.set_margin_end(10)
        self.add(self.box)

        self.start_button = Gtk.Button(label="Start")
        self.box.pack_start(self.start_button, False, False, 0)

        self.task_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=5)
        self.box.pack_start(self.task_box, True, True, 0)

        self.clock_label = Gtk.Label(label="")
        self.box.pack_end(self.clock_label, False, False, 0)

        self.connect("destroy", Gtk.main_quit)

        self.show_all()

        # Delay fetching window properties until window is realized
        GLib.idle_add(self.setup_taskbar)

        self.screen = Wnck.Screen.get_default()
        self.screen.force_update()

        # Update time and windows every second
        GLib.timeout_add(1000, self.update_clock)
        GLib.timeout_add(2000, self.update_windows)

    def setup_taskbar(self):
        # Get screen geometry (use get_monitor_at_window for the current window)
        screen = Gdk.Screen.get_default()
        monitor = screen.get_monitor_at_window(self.get_window())  # Get monitor that the current window is on
        geometry = screen.get_monitor_geometry(monitor)  # Get geometry using monitor index
        width = geometry.width
        height = geometry.height

        self.set_size_request(width, 30)
        self.move(0, height - 30)

    def update_clock(self):
        now = time.strftime("%H:%M:%S")
        self.clock_label.set_text(now)
        return True  # keep running

    def update_windows(self):
        # Clear previous taskbar buttons
        for child in self.task_box.get_children():
            self.task_box.remove(child)

        # Force update screen to get all current windows
        self.screen.force_update()
        windows = self.screen.get_windows()

        # Debug: Log all windows detected
        print("Detected windows:")
        for window in windows:
            print(f"Window: {window.get_name()} | State: {window.get_state()}")

        for window in windows:
            # Check if the window is visible and not minimized
            state = window.get_state()
            if not window.is_skip_tasklist() and not (state & Wnck.WindowState.MINIMIZED):  # Not minimized
                btn = Gtk.Button(label=window.get_name())
                icon = window.get_icon()
                if icon:
                    # Use GdkPixbuf to scale the image
                    img = Gtk.Image.new_from_pixbuf(icon.scale_simple(16, 16, GdkPixbuf.InterpType.BILINEAR))
                    btn.set_image(img)
                    btn.set_always_show_image(True)

                # Focus window on click
                btn.connect("clicked", self.on_button_clicked, window)

                # Add to taskbar
                self.task_box.pack_start(btn, False, False, 0)

        # Update the taskbar display
        self.task_box.show_all()
        return True  # keep running

    def on_button_clicked(self, widget, window):
        # Activate the window when the button is clicked
        window.activate(Wnck.WindowActivationType.CLICK)

Panel()
Gtk.main()
