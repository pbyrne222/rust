// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test a rather underspecified example:

pub fn main() {
    let f = {|&: i| i};
    assert_eq!(f(2i), 2i);
    assert_eq!(f(5i), 5i);
}
