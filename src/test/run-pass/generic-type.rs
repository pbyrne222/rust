// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



struct Pair<T> {x: T, y: T}

pub fn main() {
    let x: Pair<int> = Pair {x: 10, y: 12};
    assert_eq!(x.x, 10);
    assert_eq!(x.y, 12);
}
