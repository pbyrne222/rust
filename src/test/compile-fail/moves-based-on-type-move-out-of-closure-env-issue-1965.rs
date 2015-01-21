// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

use std::usize;

fn test(_x: Box<usize>) {}

fn main() {
    let i = box 3;
    let _f = |&:| test(i); //~ ERROR cannot move out
}
