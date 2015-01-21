// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Foo<T> {
    fn get(&self) -> T;
}

impl Foo<i32> for i32 {
    fn get(&self) -> i32 { *self }
}

fn main() {
    let x: i32 = 1;
    Foo::<i32>::get(&x);
}
