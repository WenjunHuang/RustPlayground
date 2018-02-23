extern crate chrono;
extern crate futures;
extern crate tokio_core;

use chrono::Duration;
use chrono::prelude::*;
use futures::prelude::*;
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;

#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}

impl WaitForIt {
    fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            polls: 0,
            message,
            until: Utc::now() + delay,
        }
    }
}

impl Future for WaitForIt {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let now = Utc::now();
        if self.until < now {
            Ok(Async::Ready(format!("{} after {} polls", self.message, self.polls)))
        } else {
            self.polls += 1;
            println!("not ready yet --> {:?}", self);
            futures::task::current().notify();
            Ok(Async::NotReady)
        }
    }
}

fn main() {
    let mut reactor = Core::new().unwrap();
    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    println!("wfi_1 == {:?}", wfi_1);

    let wfi_2 = WaitForIt::new("I'm done too:".to_owned(), Duration::seconds(1));
    println!("wfi_2 == {:?}", wfi_1);

    let v = vec![wfi_1,wfi_2];
    let sel = futures::future::join_all(v);

    let ret = reactor.run(sel).unwrap();
    println!("ret == {:?}", ret);
}