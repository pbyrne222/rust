// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo {
      baz: usize
}

impl Foo {
    fn bar() {
        Foo { baz: 0 }.bar();
    }

    fn bar() { //~ ERROR duplicate definition of value `bar`
    }
}

fn main() {}
