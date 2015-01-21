// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_imports, dead_code)]

use foo::Foo;

mod foo {
    pub use m::Foo; // this should shadow d::Foo
}

mod m {
    pub struct Foo;
}

mod d {
    pub struct Foo;
}

fn main() {}
