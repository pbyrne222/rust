// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::mpsc::{channel, Sender};

// tests that ctrl's type gets inferred properly
struct Command<K, V> {
    key: K,
    val: V
}

fn cache_server<K:Send,V:Send>(mut tx: Sender<Sender<Command<K, V>>>) {
    let (tx1, _rx) = channel();
    tx.send(tx1);
}
pub fn main() { }
