#![feature(conservative_impl_trait)]
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::Future;
use futures::prelude::*;
use futures::future::ok;
use std::error::Error;

fn my_fut() -> impl Future<Item=u32, Error=Box<Error+'static>> {
    ok(100)
}

fn my_fn_squared(i: u32) -> Result<u32, Box<Error+'static>> {
    Ok(i * i)
}

fn my_fut_squared(i: u32) -> impl Future<Item=u32, Error=Box<Error+'static>> {
    ok(i * i)
}

fn fut_generic_own<A>(a1: A, a2: A) -> impl Future<Item=A, Error=Box<Error+'static>>
    where A: std::cmp::PartialOrd {
    if a1 < a2 {
        ok(a1)
    } else {
        ok(a2)
    }
}

fn main() {
    let mut reactor = Core::new().unwrap();
    let future = fut_generic_own("Sampdoria","Juventus");
    let retval = reactor.run(future).unwrap();
    println!("{:?}", retval);
}
