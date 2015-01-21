// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.




fn f() -> int { return 42; }

pub fn main() {
    let g: fn() -> int = f;
    let i: int = g();
    assert_eq!(i, 42);
}
