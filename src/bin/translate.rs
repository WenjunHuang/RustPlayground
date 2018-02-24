#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(conservative_impl_trait)]
extern crate crypto;
extern crate futures;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate hyper;
extern crate percent_encoding;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate gtk_rs_playground;

use crypto::digest::Digest;
use crypto::md5::Md5;
use futures::future::*;
use futures::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use hyper::{Chunk, Client};
use hyper::client::HttpConnector;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use rand::Rng;
use serde_json::{from_slice, from_value, Value};
use std::cell::RefCell;
use std::env::args;
use std::error::Error as StdError;
use std::fmt;
use std::sync::mpsc::{channel, Receiver};
use tokio_core::reactor::Core;

const BAIDU_TRANS_URL: &str = "http://api.fanyi.baidu.com/api/trans/vip/translate";
const APP_ID: &str = "20180214000122695";
const APP_SECRET: &str = "1kxFYGCJItvLDTFAlrDe";

#[derive(Serialize, Deserialize, Debug)]
struct TransResultItem {
    src: String,
    dst: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct APISuccess {
    from: String,
    to: String,
    trans_result: Vec<TransResultItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIError {
    error_code: String,
    error_msg: String,
}

#[derive(Debug)]
enum Error {
    APIError(APIError),
    JsonError(serde_json::Error),
    JsonFormatError(String),
    HyperError(hyper::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonError(ref e) => fmt::Display::fmt(e, f),
            Error::HyperError(ref e) => fmt::Display::fmt(e, f),
            Error::JsonFormatError(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.description()),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(hyper_error: hyper::Error) -> Self {
        Error::HyperError(hyper_error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_json_error: serde_json::Error) -> Self {
        Error::JsonError(serde_json_error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIError(ref e) => &e.error_msg,
            Error::JsonError(ref e) => e.description(),
            Error::HyperError(ref e) => e.description(),
            Error::JsonFormatError(ref e) => e,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::JsonError(ref e) => Some(e),
            Error::HyperError(ref e) => Some(e),
            _ => None,
        }
    }
}

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

    let (tx, rx) = channel::<APISuccess>();
    GLOBAL.with(clone!(translate_input => move |global| {
        *global.borrow_mut() = Some((translate_input.get_buffer().unwrap(),rx))
    }));

    translate_button.connect_clicked(
        clone!(language_src,language_dst, translate_input => move |_| {
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
    }),
    );

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

macro_rules! encode {
    ($x:ident) => {
        utf8_percent_encode($x,DEFAULT_ENCODE_SET).to_string()
    };
}

fn sign(to_sign: &str) -> String {
    let mut md5 = Md5::new();
    let mut buffer: [u8; 16] = [0; 16];

    md5.input(to_sign.as_bytes());
    md5.result(&mut buffer);

    buffer
        .into_iter()
        .map(|num| format!("{:02x}", num))
        .collect::<Vec<_>>()
        .join("")
}

fn generate_url(
    input: &str,
    src: &str,
    dst: &str,
) -> impl Future<Item = hyper::Uri, Error = Error> {
    let mut rng = rand::thread_rng();
    let salt: u32 = rng.gen();
    let raw = format!(
        "{app_id}{q}{salt}{app_secret}",
        app_id = APP_ID,
        q = input,
        salt = salt,
        app_secret = APP_SECRET
    );
    let signed = sign(&raw);

    match format!(
        "{url}?q={q}&from={from}&to={to}&appid={appid}&salt={salt}&sign={sign}",
        url = BAIDU_TRANS_URL,
        from = encode!(src),
        to = encode!(dst),
        appid = APP_ID,
        q = encode!(input),
        salt = salt,
        sign = signed
    ).parse()
    {
        Ok(url) => {
            println!("{:?}", url);
            ok(url)
        }
        Err(e) => err(Error::HyperError(hyper::error::Error::Uri(e))),
    }
}

fn translate<'a>(
    input: &str,
    src: &str,
    dst: &str,
    client: &'a Client<HttpConnector>,
) -> impl Future<Item = APISuccess, Error = Error> + 'a {
    generate_url(input, src, dst)
        .and_then(move |url| client.get(url).map_err(|err| Error::HyperError(err)))
        .and_then(|res| res.body().concat2().map_err(|err| Error::HyperError(err)))
        .and_then(move |body: Chunk| {
            let v = from_slice::<Value>(&body);
            match v {
                Ok(Value::Object(m)) => {
                    if m.contains_key("error_code") {
                        match from_value::<APIError>(Value::Object(m)) {
                            Ok(result) => err(Error::APIError(result)),
                            Err(e) => err(Error::JsonError(e)),
                        }
                    } else {
                        match from_value::<APISuccess>(Value::Object(m)) {
                            Ok(result) => ok(result),
                            Err(e) => err(Error::JsonError(e)),
                        }
                    }
                },
                Ok(obj) => err(Error::JsonFormatError(format!("{:?}", obj))),
                Err(e) => err(Error::JsonError(e)),
            }
        })
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref buf, ref rx)) = *global.borrow() {
            if let Ok(text) = rx.try_recv() {
                let text = text.trans_result
                    .into_iter()
                    .map(|x| x.dst)
                    .collect::<Vec<_>>()
                    .join(",");
                buf.set_text(&text);
            }
        }
    });
    glib::Continue(false)
}

thread_local!(
static GLOBAL: RefCell < Option <(gtk::TextBuffer, Receiver < APISuccess > )> > = RefCell::new(None)
);

fn main() {
    let app = gtk::Application::new("com.wenjun.translate", gio::ApplicationFlags::empty())
        .expect("Initializing failed...");
    app.connect_startup(move |app| {
        build_ui(app);
    });
    app.connect_activate(|_| {});

    app.run(&args().collect::<Vec<_>>());
}
