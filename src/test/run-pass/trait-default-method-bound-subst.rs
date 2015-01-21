// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait A<T> {
    fn g<U>(&self, x: T, y: U) -> (T, U) { (x, y) }
}

impl A<int> for int { }
impl<T> A<T> for uint { }

fn f<T, U, V: A<T>>(i: V, j: T, k: U) -> (T, U) {
    i.g(j, k)
}

pub fn main () {
    assert_eq!(f(0i, 1i, 2i), (1, 2));
    assert_eq!(f(0u, 1i, 2i), (1, 2));
}
