// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait A {
    fn g<T>(&self, x: T, y: T) -> (T, T) { (x, y) }
}

impl A for int { }

fn f<T, V: A>(i: V, j: T, k: T) -> (T, T) {
    i.g(j, k)
}

pub fn main () {
    assert_eq!(f(0i, 1i, 2i), (1, 2));
    assert_eq!(f(0i, 1u8, 2u8), (1u8, 2u8));
}
