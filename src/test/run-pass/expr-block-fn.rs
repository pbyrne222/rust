// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



fn test_fn() {
    fn ten() -> int { return 10; }
    let rs = ten;
    assert!((rs() == 10));
}

pub fn main() { test_fn(); }
