#![feature(conservative_impl_trait)]
extern crate hyper;
extern crate gtk;
extern crate gio;
extern crate rand;
extern crate crypto;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
extern crate glib;
extern crate url;

#[macro_use]
extern crate gtk_rs_playground;

use gio::prelude::*;
use gtk::prelude::*;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::env::args;
use rand::Rng;
use hyper::{Client, Chunk};
use hyper::client::HttpConnector;
use futures::future::*;
use futures::prelude::*;
use url::Url;
use tokio_core::reactor::Core;
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;

const BAIDU_TRANS_URL: &str = "http://api.fanyi.baidu.com/api/trans/vip/translate";
const APP_ID: &str = "20180214000122695";
const APP_SECRET: &str = "1kxFYGCJItvLDTFAlrDe";

fn build_ui(app: &gtk::Application) {
    let ui_source = include_str!("translate.glade");
    let builder = gtk::Builder::new_from_string(ui_source);

    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    let translate_button: gtk::Button = builder.get_object("translate_button").unwrap();
    let translate_input: gtk::TextView = builder.get_object("translation_input").unwrap();
    let language_dst: gtk::ComboBoxText = builder.get_object("language_dst").unwrap();
    let language_src: gtk::ComboBoxText = builder.get_object("language_src").unwrap();

    let input_buffer = gtk::TextBuffer::new(None);
    translate_input.set_buffer(Some(&input_buffer));

    let (tx, rx) = channel();
    GLOBAL.with(clone!(translate_input => move |global| {
        *global.borrow_mut() = Some((translate_input.get_buffer().unwrap(),rx))
    }));

    translate_button.connect_clicked(clone!(language_src,language_dst, translate_input => move |_| {
        let buffer = translate_input.get_buffer().unwrap();
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();
        let string = buffer.get_text(&start, &end, false).unwrap();
        let src = match_language(language_src.get_active_text().unwrap().as_str());
        let dst = match_language(language_dst.get_active_text().unwrap().as_str());

        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        // translate
        let fut = translate(&string,&src, &dst, &client).and_then(|result|{
              tx.send(result).expect("Couldn't send data to channel");
              glib::idle_add(receive);
              ok(())
        });
        core.run(fut).unwrap();
    }));

    window.set_application(app);
    window.show_all();
}

fn match_language(input: &str) -> String {
    match input {
        "中文" => "zh".to_owned(),
        "英语" => "en".to_owned(),
        "日语" => "jp".to_owned(),
        "粤语" => "yue".to_owned(),
        "文言文" => "wyw".to_owned(),
        "韩语" => "kor".to_owned(),
        _ => {
            println!("Language not supported");
            std::process::exit(1);
        }
    }
}

fn translate(input: &str, src:&str, dst: &str, client: &Client<HttpConnector>) -> impl Future<Item=String, Error=hyper::Error> {
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
        let mut output: [u8; 16] = [0; 16];
        hasher.result(&mut output);
        signed = output.into_iter().map(|num| {
            format!("{:02x}", num)
        }).collect::<Vec<_>>().join("");
    }

    let url = Url::parse_with_params(BAIDU_TRANS_URL,&[("from",&src])
    let complete_url = format!("{url}?q={q}&from={from}&to={to}&appid={appid}&salt={salt}&sign={sign}",
                               url = BAIDU_TRANS_URL,
                               from = src,
                               to = dst,
                               appid = APP_ID,
                               q = input,
                               salt = salt,
                               sign = signed);
    println!("{}", complete_url);

    let complete_url = complete_url.parse().unwrap();
    let work = client.get(complete_url)
        .map(|res| {
            res.body().concat2().map(move |body: Chunk| {
                println!("{:?}",body);
                let v = serde_json::from_slice::<serde_json::Value>(&body);
                match v {
                    Ok(serde_json::Value::Object(m)) => {
                        if let Some(error) = m.get("error_code") {
                            println!("Error");
                            format!("{}", error)
                        } else {
                            let trans_result = m.get("trans_result").unwrap();
                            let trans_result = match *trans_result {
                                serde_json::Value::Array(ref trans) => {
                                    if let serde_json::Value::Object(ref translate_str) = trans[0] {
                                        let translate_str = translate_str.get("dst").unwrap();
                                        if let serde_json::Value::String(ref translate_str) = *translate_str {
                                            println!("{}", translate_str);
                                            translate_str.clone()
                                        } else {
                                            "".to_owned()
                                        }
                                    } else {
                                        println!("failed");
                                        "".to_owned()
                                    }
                                }
                                _ => {
                                    println!("not correct type");
                                    "".to_owned()
                                }
                            };
                            println!("Success");
                            format!("{}", trans_result)
                        }
                    }
                    _ => {
                        "Error".to_owned()
                    }
                }
            })
        }).flatten();
    work
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref buf, ref rx)) = *global.borrow() {
            if let Ok(text) = rx.try_recv() {
                buf.set_text(&text);
            }
        }
    });
    glib::Continue(false)
}

thread_local!(
    static GLOBAL: RefCell<Option<(gtk::TextBuffer,Receiver<String>)>>  = RefCell::new(None)
);

fn main() {
    let app = gtk::Application::new("com.wenjun.translate", gio::ApplicationFlags::empty()).expect("Initializing failed...");
    app.connect_startup(move |app| {
        build_ui(app);
    });
    app.connect_activate(|_| {});

    app.run(&args().collect::<Vec<_>>());
}