// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Make sure const bounds work on things, and test that a few types
// are const.

#![allow(unknown_features)]
#![feature(box_syntax)]

fn foo<T: Sync>(x: T) -> T { x }

struct F { field: int }

pub fn main() {
    /*foo(1);
    foo("hi".to_string());
    foo(~[1, 2, 3]);
    foo(F{field: 42});
    foo((1, 2u));
    foo(@1);*/
    foo(box 1i);
}
