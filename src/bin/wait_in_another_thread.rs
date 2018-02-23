extern crate chrono;
extern crate tokio_core;
extern crate futures;

use chrono::prelude::*;
use chrono::Duration;
use futures::task::{self,Task};
use std::thread;
use tokio_core::reactor::Core;
use futures::prelude::*;
use std::error::Error;

pub struct WaitInAnotherThread{
    end_time:DateTime<Utc>,
    running:bool,
}

impl WaitInAnotherThread{
    pub fn new(how_long:Duration) -> WaitInAnotherThread {
        WaitInAnotherThread{
            end_time: Utc::now() + how_long,
            running: false,
        }
    }

    pub fn wait_spin(&self) {
        while Utc::now() < self.end_time{}
        println!("the time has come == {:?}!", self.end_time);
    }

    pub fn wait_blocking(&self){
        while Utc::now() < self.end_time {
            let delta_sec = self.end_time.timestamp() - Utc::now().timestamp();
            if delta_sec > 0{
                thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
            }
        }
        println!("the time has come == {:?}!",self.end_time);
    }

    pub fn run(&mut self,task:Task) {
        let lend = Clone::clone(&self.end_time);

        thread::spawn(move ||{
            while Utc::now() < lend {
                let delta_sec = lend.timestamp() - Utc::now().timestamp();
                if delta_sec > 0 {
                    thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
                }
                task.notify();
            }
            println!("the time has come == {:?}!", lend);
        });
    }
}

impl Future for WaitInAnotherThread {
    type Item = ();
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        if Utc::now() < self.end_time {
            println!("not ready yet! parking the task.");

            if !self.running {
                println!("side thread not running starting now!");
                self.run(task::current());
                self.running = true;
            }
            Ok(Async::NotReady)
        }else {
            println!("ready! the task will complete.");
            Ok(Async::Ready(()))
        }
    }
}

fn main(){
    let mut reactor = Core::new().unwrap();
    let wiat = WaitInAnotherThread::new(Duration::seconds(3));
    println!("wait future started");
    let ret = reactor.run(wiat).unwrap();
    println!("wait future completed. ret=={:?}",ret);
}