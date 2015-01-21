// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

static mut a: Box<isize> = box 3;
//~^ ERROR statics are not allowed to have custom pointers
//~^^ ERROR mutable statics are not allowed to have owned pointers

fn main() {}
