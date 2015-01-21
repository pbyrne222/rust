// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:fail

#![allow(unknown_features)]
#![feature(box_syntax)]

fn failfn() {
    panic!();
}

fn main() {
    box 0i;
    failfn();
}
