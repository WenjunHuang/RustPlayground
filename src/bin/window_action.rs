extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;

fn main(){
  let app = gtk::Application::new("com.wenjun.windowaction",gio::ApplicationFlags::FLAGS_NONE).unwrap();
  app.connect_activate(|app|{
    let window = gtk::ApplicationWindow::new(app);
    let button = gtk::Button::new_with_label("Click");

    let action = gio::SimpleAction::new("save",None);
    action.connect_activate(|action,param|{
      println!("Hello Rust");
    });

    button.set_action_name("win.save");
    window.add(&button);
    window.add_action(&action);
    window.show_all();
  });
  app.run(&args().collect::<Vec<_>>());
}