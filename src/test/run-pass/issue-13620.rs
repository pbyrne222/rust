// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:issue-13620-1.rs
// aux-build:issue-13620-2.rs

extern crate "issue-13620-2" as crate2;

fn main() {
    (crate2::FOO2.foo)();
}
