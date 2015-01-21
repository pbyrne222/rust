// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:issue-21202.rs

extern crate "issue-21202" as crate1;

use crate1::A;

mod B {
    use crate1::A::Foo;
    fn bar(f: Foo) {
        Foo::foo(&f);
        //~^ ERROR: function `foo` is private
    }
}

fn main() { }
