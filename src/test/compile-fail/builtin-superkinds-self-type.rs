// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests (negatively) the ability for the Self type in default methods
// to use capabilities granted by builtin kinds as supertraits.

use std::sync::mpsc::{channel, Sender};

trait Foo : Sync+'static {
    fn foo(self, mut chan: Sender<Self>) { }
}

impl <T: Sync> Foo for T { }
//~^ ERROR the parameter type `T` may not live long enough

fn main() {
    let (tx, rx) = channel();
    1193182is.foo(tx);
    assert!(rx.recv() == 1193182is);
}
