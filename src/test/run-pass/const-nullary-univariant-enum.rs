// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum Foo {
    Bar = 0xDEADBEE
}

impl Copy for Foo {}

static X: Foo = Foo::Bar;

pub fn main() {
    assert_eq!((X as uint), 0xDEADBEE);
    assert_eq!((Y as uint), 0xDEADBEE);
}

static Y: Foo = Foo::Bar;
