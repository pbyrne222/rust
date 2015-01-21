// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host

#![feature(plugin_registrar)]
#![feature(box_syntax)]

extern crate rustc;

use std::any::Any;
use std::cell::RefCell;
use rustc::plugin::Registry;

struct Foo {
    foo: int
}

impl Drop for Foo {
    fn drop(&mut self) {}
}

#[plugin_registrar]
pub fn registrar(_: &mut Registry) {
    thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
    FOO.with(|s| *s.borrow_mut() = Some(box Foo { foo: 10 } as Box<Any+Send>));
}

