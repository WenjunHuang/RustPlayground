extern crate futures;

use std::io;
use std::time::Duration;
use futures::prelude::*;
use futures::future::Map;

fn add_ten<F>(future: F) -> Map<F, fn(i32) -> i32>
    where F: Future<Item=i32> {
    fn add(a: i32) -> i32 { a + 10 }
    future.map(add)
}

fn add<'a, A, B>(a: A, b: B) -> Box<Future<Item=i32, Error=A::Error>>
    where A: Future<Item=i32> + 'a,
          B: Future<Item=i32, Error=A::Error> + 'a {
    Box::new(a.join(b).map(|(a, b)| a + b))
}

fn download_timeout(url: &str,
                    timeout_dur: Duration) -> Box<Future<Item=Vec<u8>, Error=io::Error>> {
    use std::io;
    use std::net::{SocketAddr,TcpStream};

    type IoFuture<T> = Box<Future<Item=T,Error=io::Error>>;


}