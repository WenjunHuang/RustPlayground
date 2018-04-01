#![feature(proc_macro)]
extern crate glib_sys;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate json;

#[macro_use]
extern crate relm;
extern crate relm_attributes;

#[macro_use]
extern crate relm_derive;
extern crate simplelog;
extern crate uhttp_uri;

use std::cell::RefCell;
use std::mem;
use gdk::RGBA;
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::*;
use glib::Cast;
use glib::source::PRIORITY_DEFAULT;
use gtk::*;
use gtk::Orientation::Vertical;
use relm::*;
use relm_attributes::widget;
use simplelog::*;
use uhttp_uri::HttpUri;

use self::Msg::*;
use self::HttpMsg::*;

const RED: &RGBA = &RGBA { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 };
const READ_SIZE: usize = 1024;

pub struct Model{
    button_enabled: bool,
    loader: PixbufLoader,
    relm: Relm<Win>,
    topic : String,
    text: String,
}

#[derive(Msg)]
pub enum Msg {
    DownloadCompleted,
    FetchUrl,
    HttpError(String),
    ImageChunk(Vec<u8>),
    NewGif(Vec<u8>),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>,():()) -> Model {
        let tops = "cats";
        Model {
            button_enabled: true,
            loader: PixbufLoader::new(),
            relm: relm.clone(),
            topic: topic.to_string(),
            text: topic.to_string(),
        }
    }

    fn update(&mut self, event:Msg){
        match event {
            DownloadCompleted => {
                self.model.button_enabled = true;
                self.button.grab_focus();
                self.model.loader.close().unwrap();
                self.image.set_from_pixbuf(self.model.get_pixbuf().as_ref());
            },
            FetchUrl => {
                self.model.text = String::new();
                self.model.button_enabled = false;

                let url = format!("https://api.giphy.com/v1/gifs/random?api_key=dc6zaTOxFJmzC&tag={}",
                self.model.topic);
                let http = execute::<Http>(url);
                connect_stream!(http@ReadDone(ref buffer),self.model.relm.stream(),NewGif(buffer.take()));
            },
            HttpError(error) => {
                self.model.button_enabled = true;
                self.model.text = format!("HTTP error: {}",error);
                self.label.override_color(StateFlags::NORMAL,RED);
            },
            ImageChunk(chunk) => {
                if let Erro(error) = self.model.loader.write(&chunk) {
                    println!("{}",error);
                }
            },
            NewGif(buffer) => {
                if let Ok(body) = String::from_utf8(buffer) {
                    let mut json = json::parse(&body).unwrap();
                    let url = json["data"]["image_url"].take_string().unwrap();
                    let http = execute::<Http>(url);
                    connect_stream!(http@DataRead(ref buffer),self.model.relm.stream(),ImageChunk(buffer.take()));
                    connect_stream!(http@ReadDone(_),self.model.relm.stream(),DownloadCompleted);
                }
            },
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="label"]
                gtk::Label {
                    text: &self.model.text,
                },
                #[name="image"]
                gtk::Image {
                },
                #[name="button"]
                gtk::Button {
                    label: "Load image",
                    sensitive: self.model.button_enabled,
                    clicked => FetchUrl,
                },
            },
            delete_event(_,_) => (Quit,Inhibit(false)),
        }
    }
}

impl Drop for Win {
    fn drop(&mut self){
        self.model.loader.close().ok();
    }
}

struct HttpModel {
    buffer: Vec<u8>,
    found_crlf: bool,
    relm: Relm<Htpt>,
    stream: Option<IOStream>,
    url: String,
}

struct Bytes{
    bytes: RefCell<Option<Vec<u8>>>,
}

impl Bytes{
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: RefCell::new(Some(bytes)),
        }
    }

    fn take(&self) -> Vec<u8>{
        self.bytes.borrow_mut().take().unwrap_or_default()
    }
}

#[derive(Msg)]
enum HttpMsg {
    Connection(SocketConnection),
    DataRead(Bytes),
    Read((Vec<u8>,usize)),
}

unsafe impl Send for HttpMsg{}

struct Http{
    model:HttpModel,
}

impl Update for Http {
    type Model = HttpModel;
    type ModelParam = String;
    type Msg = HttpMsg;
}

fn main() {}