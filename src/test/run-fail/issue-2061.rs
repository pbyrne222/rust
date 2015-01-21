// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-test
// error-pattern: task '<main>' has overflowed its stack

struct R {
    b: int,
}

impl Drop for R {
    fn drop(&mut self) {
        let _y = R { b: self.b };
    }
}

fn main() {
    let _x = R { b: 0 };
}
