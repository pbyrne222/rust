// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:pub_use_xcrate1.rs
// aux-build:pub_use_xcrate2.rs

extern crate pub_use_xcrate2;

use pub_use_xcrate2::Foo;

pub fn main() {
    let _foo: Foo = Foo {
        name: 0
    };
}
