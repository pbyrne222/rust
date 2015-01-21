// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:trait-safety-lib.rs

// Check that unsafe traits require unsafe impls and that inherent
// impls cannot be unsafe.

extern crate "trait-safety-lib" as lib;

struct Bar;
impl lib::Foo for Bar { //~ ERROR requires an `unsafe impl` declaration
    fn foo(&self) -> isize {
        *self as isize
    }
}

fn main() { }
