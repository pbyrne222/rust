// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:issue_19293.rs
extern crate issue_19293;
use issue_19293::{Foo, MyEnum};

fn main() {
    MyEnum::Foo(Foo(5));
}
