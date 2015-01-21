// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo {
    x: int
}

impl Foo {
    pub fn new() -> Foo {
        Foo { x: 3 }
    }
}

pub fn main() {
    let x = Foo::new();
    println!("{}", x.x);
}
