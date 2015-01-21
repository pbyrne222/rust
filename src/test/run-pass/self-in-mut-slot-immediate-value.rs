// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Assert that `mut self` on an immediate value doesn't
// allow mutating the original - issue #10615.

struct Value {
    n: int
}

impl Copy for Value {}

impl Value {
    fn squared(mut self) -> Value {
        self.n *= self.n;
        self
    }
}

pub fn main() {
    let x = Value { n: 3 };
    let y = x.squared();
    assert_eq!(x.n, 3);
    assert_eq!(y.n, 9);
}
