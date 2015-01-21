// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time::Duration;
use std::thread::Thread;

fn main() {
    Thread::scoped(move|| customtask()).join().ok().unwrap();
}

fn customtask() {
    let mut timer = std::io::timer::Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(10));
    periodic.recv();
}
