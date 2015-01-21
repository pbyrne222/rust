// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    let x = [1i; 100];
    let mut y = 0i;
    for i in x.iter() {
        if y > 10 {
            break;
        }
        y += *i;
    }
    assert!(y == 11);
}
