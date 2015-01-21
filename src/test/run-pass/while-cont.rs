// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Issue #825: Should recheck the loop condition after continuing
pub fn main() {
    let mut i = 1i;
    while i > 0 {
        assert!((i > 0));
        println!("{}", i);
        i -= 1;
        continue;
    }
}
