// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct r;

impl Drop for r {
    fn drop(&mut self) {
        true //~  ERROR mismatched types
             //~| expected ()
             //~| found bool
             //~| expected ()
             //~| found bool
    }
}

fn main() {
}
