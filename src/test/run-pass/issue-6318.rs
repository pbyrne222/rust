// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

pub enum Thing {
    A(Box<Foo+'static>)
}

pub trait Foo {}

pub struct Struct;

impl Foo for Struct {}

pub fn main() {
    match Thing::A(box Struct as Box<Foo+'static>) {
        Thing::A(_a) => 0i,
    };
}

