// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::mpsc::{TryRecvError, channel};
use std::io::timer::Timer;
use std::thread::Thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = channel();
    let _t = Thread::scoped(move||{
        let mut timer = Timer::new().unwrap();
        timer.sleep(Duration::milliseconds(10));
        tx.send(()).unwrap();
    });
    loop {
        match rx.try_recv() {
            Ok(()) => break,
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => unreachable!()
        }
    }
}
