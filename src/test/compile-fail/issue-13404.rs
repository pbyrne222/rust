// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use a::f;
use b::f;
//~^ ERROR: unresolved import `b::f`. There is no `f` in `b`

mod a { pub fn f() {} }
mod b { }

fn main() {
    f();
}
