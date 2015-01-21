// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn foo(x: isize) { println!("{}", x); }

fn main() {
    let x: isize; if 1is > 2 { x = 10; }
    foo(x); //~ ERROR use of possibly uninitialized variable: `x`
}
