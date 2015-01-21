// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

fn dup(x: Box<isize>) -> Box<(Box<isize>,Box<isize>)> { box() (x, x) } //~ ERROR use of moved value
fn main() {
    dup(box 3);
}
