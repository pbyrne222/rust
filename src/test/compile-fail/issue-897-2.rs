// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(unreachable_code)]

fn g() -> ! { panic!(); }
fn f() -> ! {
    return g(); //~ ERROR `return` in a function declared as diverging
    g();
}
fn h() -> ! {
    loop {}
    g();
}

fn main() { f() }
