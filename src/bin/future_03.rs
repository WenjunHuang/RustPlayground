#![feature(conservative_impl_trait)]
extern crate futures;
extern crate tokio_core;

use std::fmt;
use std::error;
use std::convert::From;
use tokio_core::reactor::Core;
use futures::prelude::*;
use futures::future::*;

#[derive(Debug, Default)]
pub struct ErrorA {}

impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorA!")
    }
}

impl error::Error for ErrorA {
    fn description(&self) -> &str {
        "Description for ErrorA"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, Default)]
pub struct ErrorB {}

impl fmt::Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorB!")
    }
}

impl error::Error for ErrorB {
    fn description(&self) -> &str {
        "Description for ErrorB"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<ErrorB> for ErrorA {
    fn from(e: ErrorB) -> ErrorA {
        ErrorA::default()
    }
}

impl From<ErrorA> for ErrorB {
    fn from(e: ErrorA) -> ErrorB {
        ErrorB::default()
    }
}

fn fut_error_a() -> impl Future<Item=(), Error=ErrorA> {
    err(ErrorA {})
}

fn fut_error_b() -> impl Future<Item=(), Error=ErrorB> {
    err(ErrorB {})
}

fn my_fn_ref<'a>(s:&'a str) -> Result<&'a str,Box<error::Error>> {
    Ok(s)
}

fn my_fut_ref_implicit(s:&str) -> impl Future<Item = &str,Error=Box<error::Error>> {
    ok(s)
}

fn my_fut_ref_chained(s:&str) ->impl Future<Item=String,Error=Box<error::Error>> {
    my_fut_ref_implicit(s).and_then(|s| ok(format!("received = {}",s)))
}

fn main() {
    let mut reactor = Core::new().unwrap();
    let retval = reactor.run(fut_error_a()).unwrap_err();
    println!("fut_error_a == {:?}", retval);

    let retval = reactor.run(fut_error_b()).unwrap_err();
    println!("fut_error_b == {:?}", retval);

    let future = fut_error_a()
        .from_err()
        .and_then(|_| fut_error_b());
    let retval = reactor.run(future).unwrap_err();
    println!("error chain == {:?}", retval);

    let retval = reactor.run(my_fut_ref_chained("str with lifetime")).unwrap();
    println!("my_fut_ref_chained == {}",retval);
}
