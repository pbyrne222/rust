// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::mpsc::channel;

pub fn main() { test00(); }

fn test00() {
    let _r: int = 0;
    let mut sum: int = 0;
    let (tx, rx) = channel();
    let number_of_messages: int = 1000;
    let mut i: int = 0;
    while i < number_of_messages { tx.send(i + 0).unwrap(); i += 1; }
    i = 0;
    while i < number_of_messages { sum += rx.recv().unwrap(); i += 1; }
    assert_eq!(sum, number_of_messages * (number_of_messages - 1) / 2);
}
