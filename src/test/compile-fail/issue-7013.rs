// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

use std::cell::RefCell;
use std::rc::Rc;

trait Foo {
    fn set(&mut self, v: Rc<RefCell<A>>);
}

struct B {
    v: Option<Rc<RefCell<A>>>
}

impl Foo for B {
    fn set(&mut self, v: Rc<RefCell<A>>)
    {
        self.v = Some(v);
    }
}

struct A {
    v: Box<Foo + Send>,
}

fn main() {
    let a = A {v: box B{v: None} as Box<Foo+Send>};
    //~^ ERROR the trait `core::marker::Send` is not implemented
}
