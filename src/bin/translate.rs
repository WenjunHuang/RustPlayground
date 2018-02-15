extern crate hyper;
extern crate gtk;
extern crate gio;
extern crate rand;
extern crate crypto;
extern crate tokio_core;
extern crate futures;
#[macro_use]
extern crate gtk_rs_playground;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::*;
use gio::*;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::env::args;
use rand::Rng;
use hyper::Client;
use futures::{Future,Stream};
use tokio_core::reactor::Core;
use hyper::header::Connection;

const BAIDU_TRANS_URL: &str = "http://api.fanyi.baidu.com/api/trans/vip/translate";
const APP_ID: &str = "20180214000122695";
const APP_SECRET: &str = "1kxFYGCJItvLDTFAlrDe";

fn build_ui(app: &gtk::Application) {
  let ui_source = include_str!("translate.glade");
  let builder = gtk::Builder::new_from_string(ui_source);

  let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
  let translate_button: gtk::Button = builder.get_object("translate_button").unwrap();
  let translate_input: gtk::TextView = builder.get_object("translation_input").unwrap();
  let language_box: gtk::ComboBoxText = builder.get_object("language").unwrap();

  let input_buffer = gtk::TextBuffer::new(None);
  translate_input.set_buffer(Some(&input_buffer));

  translate_button.connect_clicked(clone!(language_box,translate_input => move |_|{
      let buffer = translate_input.get_buffer().unwrap();
      let start = buffer.get_start_iter();
      let end = buffer.get_end_iter();
      let string = buffer.get_text(&start,&end,false).unwrap();
      let language = match_language(language_box.get_active_text().unwrap().as_str());

      // translate
      let translation = translate(&string,&language);
      translate_input.get_buffer().unwrap().set_text(translation.as_str());
  }));

  window.set_application(app);
  window.show_all();
}

fn match_language(input: &str) -> String {
  match input {
    "Chinese" => "zh".to_owned(),
    "English" => "en".to_owned(),
    _ => {
      println!("Language not supported");
      std::process::exit(1);
    }
  }
}

fn translate(input: &str, language: &str) -> String {
  let mut rng = rand::thread_rng();
  let salt: u32 = rng.gen();
  let sign = format!("{app_id}{q}{salt}{app_secret}",
                     app_id = APP_ID,
                     q = input,
                     salt = salt,
                     app_secret = APP_SECRET);
  let signed;
  {
    let mut hasher = Md5::new();
    hasher.input(sign.as_bytes());
//    hasher.input(b"2015063000000001apple143566028812345678");
    let mut output: [u8; 16] = [0; 16];
    hasher.result(&mut output);
    signed = output.into_iter().map(|num| {
      format!("{:02x}", num)
    }).collect::<Vec<_>>().join("");
  }

  let complete_url = format!("{url}?q={q}&from={from}&to={to}&appid={appid}&salt={salt}&sign={sign}",
                             url = BAIDU_TRANS_URL,
                             from = "en",
                             to = language,
                             appid = APP_ID,
                             q = input,
                             salt = salt,
                             sign = signed);
  println!("{}",complete_url);

  let mut result = String::new();
  let mut core = Core::new().unwrap();
  let client = Client::new(&core.handle());
  let complete_url = complete_url.parse().unwrap();
//  if let Ok(mut response) = client.get(complete_url).header(Connection::close()).send() {
//    if let Err(error) = response.read_to_string(&mut result) {
//      panic!("Unable to read response: {}", error);
//    }
//  }
  let work = client.get(complete_url).map(|res|{
    println!("Response: {}",res.status());
  });
  core.run(work);
  "".to_owned()
//  parse_message(&result);
}

fn parse_message(input: &str) {}


fn main() {
  let app = gtk::Application::new("com.wenjun.translate", gio::ApplicationFlags::empty()).expect("Initializing failed...");
  app.connect_startup(move |app| {
    build_ui(app);
  });
  app.connect_activate(|_| {});

  app.run(&args().collect::<Vec<_>>());
}