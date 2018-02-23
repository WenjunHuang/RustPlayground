extern crate futures;
extern crate tokio_core;

use futures::prelude::*;
use std::error::Error;
use tokio_core::reactor::Core;
use futures::future::*;

struct MyStream {
    current: u32,
    max: u32,
}

impl MyStream {
    pub fn new(max: u32) -> MyStream {
        MyStream {
            current: 0,
            max: max,
        }
    }
}

impl Stream for MyStream {
    type Item = u32;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.current {
            ref mut x if *x < self.max => {
                *x = *x + 1;
                Ok(Async::Ready(Some(*x)))
            }
            _ => Ok(Async::Ready(None)),
        }
    }
}

fn main(){
    let mut reactor = Core::new().unwrap();
    let my_stream = MyStream::new(5);
    let fut = my_stream.for_each(|num|{
        println!("num == {}", num);
        ok(())
    });

    reactor.run(fut).unwrap();
}