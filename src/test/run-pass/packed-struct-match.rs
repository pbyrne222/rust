// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[repr(packed)]
struct Foo {
    bar: u8,
    baz: uint
}

pub fn main() {
    let foo = Foo { bar: 1, baz: 2 };
    match foo {
        Foo {bar, baz} => {
            assert_eq!(bar, 1);
            assert_eq!(baz, 2);
        }
    }
}
