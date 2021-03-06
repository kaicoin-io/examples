//! # Clock Sample
//!
//! This sample demonstrates how to use gtk::timeout_add_seconds to run
//! a periodic task, implementing a clock in this example.

extern crate gio;
extern crate gtk;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Clock");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(260, 40);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let time = current_time();
    let label = gtk::Label::new(None);
    label.set_text(&time);

    window.add(&label);

    window.show_all();

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        // we could return gtk::Continue(false) to stop our clock after this tick
        gtk::Continue(true)
    };

    // executes the closure once every second
    gtk::timeout_add_seconds(1, tick);
}

fn main() {
    let application = gtk::Application::new("com.github.clock",
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
