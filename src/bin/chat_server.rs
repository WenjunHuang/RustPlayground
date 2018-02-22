#[macro_use]
extern crate futures;
extern crate tokio;

#[macro_use]
extern crate tokio_io;
extern crate bytes;

use tokio::executor::current_thread;
use tokio::net::{TcpListener, TcpStream};
use tokio_io::AsyncRead;
use futures::prelude::*;
use futures::sync::mpsc;
use futures::future::{self, Either};
use bytes::{BytesMut, Bytes, BufMut};

use std::io::{self, Write};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::rc::Rc;

type Tx = mpsc::UnboundedSender<Bytes>;
type Rx = mpsc::UnboundedReceiver<Bytes>;

struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }
}

struct Lines {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl Lines {
    fn new(socket: TcpStream) -> Self {
        Lines {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    fn fill_read_buf(&mut self) -> Result<Async(),io::Error> {
        loop {
            self.rd.reserve(1024);

            let n = try_ready!(self.socket.read_buf(&mu self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }

    fn buffer(&mut self,line: &[u8]) {
        self.wr.put(line);
    }

    fn poll_flush(&mut self) ->Result<Async<()>,io::Error> {
        while !self.wr.is_empty() {
            let n = try_nb!(self.socket.write(&self.wr));
            assert!(n > 0);

            // This discards the first 'n' bytes of the buffer.
            let _ = self.wr.split_to(n);
        }
        Ok(Async::Ready(()))
    }
}

impl Stream for Lines {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        let sock_closed = self.fill_read_buf()?.is_ready();

        let pos = self.rd.windows(2).enumerate()
            .find(|&(_, bytes)| bytes == b"\r\n")
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            let mut line = self.rd.split_to(pos + 2);
            line.split_off(pos);
            return Ok(Async::Ready(Some(line)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

struct Peer {
    /// Name of the peer. This is the first line received from the client.
    name:BytesMut,
    ///The TCP socket wrapped with the `Lines` codec.
    lines:Lines,

    /// Handle to the shared chat state.
    state:Rc<RefCell<Shared>>,

    ///Receive half of the message channel.
    rx:Rx,

    /// Client socket address.
    addr:SocketAddr,
}

impl Peer {
    fn name(name:BytesMut,state:Rc<RefCell<Shared>>,lines:Lines) -> Self{
        let addr = lines.socket.peer_addr().unwrap();
        let (tx,rx) = mpsc::unbounded();
        state.borrow_mut().peers.insert(addr,tx);

        Peer {
            name,
            lines,
            state,
            rx,
            addr,
        }
    }
}

impl Drop for Peer{
    fn drop(&mut self) {
        self.state.borrow_mut().peers.remove(&self.addr);
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(),io::Error> {
        // Receive all messages from peers.
        loop {
            // Polling an `UnboundedReceiver` cannot fail, so `unwrap`
            // here is safe.
            match self.rx.poll().unwrap() {
                Async::Ready(Some(v)) => {
                    self.lines.buffer(&v);
                },
                _=>break,
            }
        }

        // Flush the write buffer to the socket
        let _ = self.lines.poll_flush()?;

        // Read new lines from the socket
        while let Async::Ready(line) = self.lines.poll()? {
            println!("Received line ({:?} : {:?}",self.name, line);

            if let Some(message) = line {
                // Append the peer's name to the front of the line:
                let mut line = self.name.clone();
                line.put(": ");
                line.put(&message);
                lint.put("\r\n");

                let line = line.freeze();
                for (addr,tx) in &self.state.borrow().peers {
                    if *addr != self.addr {
                        tx.unbounded_send(line.clone()).unwrap();
                    }
                }
            }else {
                return Ok(Async::Ready(()));
            }
        }
        Ok(Async::NotReady)
    }
}

fn main() {
    let state = Rc::new(RefCell::new(Shared::new()));
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket, state.clone());
        OK(())
    })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            println!("accept error = {:?}", err);
        });

    // Start the executor and spawn the server task.
    current_thread::run(|_| {
        current_thread::spawn(server);
        println!("server running on localhost:6142");
    })
}

fn process(socket: TcpStream, state: Rc<RefCell<Shared>>) {
    let lines = Lines::new(socket);
    let connection = lines.into_future()
        .map_err(|(e,_)| e)
        .and_then(|(name,lines)| {
            let name = match name {
                Some(name)=>name,
                none => {
                    return Either::A(future::ok(()));
                }
            };
            println!("`{:?}` is joining the chat",name);

        });
    // Define the task that processes the connection.
    let task = unimplemented!();

    // Spawn the task
    current_thread::spawn(task);
}