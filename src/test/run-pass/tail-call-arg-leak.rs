// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.




// use of tail calls causes arg slot leaks, issue #160.
fn inner(dummy: String, b: bool) { if b { return inner(dummy, false); } }

pub fn main() {
    inner("hi".to_string(), true);
}
