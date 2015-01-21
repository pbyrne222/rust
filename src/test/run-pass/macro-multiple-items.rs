// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty - token trees can't pretty print

macro_rules! make_foo {
    () => (
        struct Foo;

        impl Foo {
            fn bar(&self) {}
        }
    )
}

make_foo!();

pub fn main() {
    Foo.bar()
}
