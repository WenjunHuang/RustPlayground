extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate glib_sys;

#[macro_use]
extern crate gtk_rs_playground;

mod model;

use std::env::args;
use gtk::prelude::*;
use gio::prelude::*;
use gio::ApplicationExt;

fn build_ui(app: &gtk::Application) {
  let window = gtk::ApplicationWindow::new(app);
  window.set_default_size(480, 360);
  window.set_title("fish bowl");

  window.show_all();
}

fn main() {
  let application = gtk::Application::new("com.wenjun.fishbowl",
                                          gio::ApplicationFlags::empty()).unwrap();
  application.connect_startup(|app| {
    build_ui(app)
  });
  application.connect_activate(|_| {});

  application.run(&args().collect::<Vec<_>>());
}