// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_assignment)]
#![allow(unused_variable)]

enum thing { a, b, c, }

fn foo<F>(it: F) where F: FnOnce(int) { it(10); }

pub fn main() {
    let mut x = true;
    match thing::a {
      thing::a => { x = true; foo(|_i| { } ) }
      thing::b => { x = false; }
      thing::c => { x = false; }
    }
}
