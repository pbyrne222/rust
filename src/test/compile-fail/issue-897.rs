// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(unreachable_code)]

fn f() -> ! {
    return panic!(); //~ ERROR `return` in a function declared as diverging
    panic!(); // the unreachable statement error is in <std macro>, at this line, there
             // only is a note
}

fn main() { f() }
