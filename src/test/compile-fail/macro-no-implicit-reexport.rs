// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:macro_reexport_1.rs
// aux-build:macro_non_reexport_2.rs
// ignore-stage1

#[macro_use] #[no_link]
extern crate macro_non_reexport_2;

fn main() {
    assert_eq!(reexported!(), 3us);  //~ ERROR macro undefined
}
