// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct C {
    x: isize,
}

impl Drop for C {
    fn drop(&mut self) {
        println!("dropping: {}", self.x);
    }
}

fn main() {
    let c = C{ x: 2};
    let _d = c.clone(); //~ ERROR does not implement any method in scope
}
