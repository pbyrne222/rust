// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

struct S {
    x: Box<isize>,
}

impl S {
    pub fn foo(self) -> isize {
        self.bar();
        return *self.x;  //~ ERROR use of moved value: `*self.x`
    }

    pub fn bar(self) {}
}

fn main() {
    let x = S { x: box 1 };
    println!("{}", x.foo());
}
