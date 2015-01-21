// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(unreachable_code)]
pub fn expr_break_label_15() {
    let mut x = 15is;
    let mut y = 151is;
    'outer: loop {
        'inner: loop {
            if x == 1is {
                break 'outer;
                "unreachable";
            }
            if y >= 2is {
                break;
                "unreachable";
            }
            y -= 3is;
        }
        y -= 4is;
        x -= 5is;
    }
}
