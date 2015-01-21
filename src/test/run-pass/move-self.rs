// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct S {
    x: String
}

impl S {
    pub fn foo(self) {
        self.bar();
    }

    pub fn bar(self) {
        println!("{}", self.x);
    }
}

pub fn main() {
    let x = S { x: "Hello!".to_string() };
    x.foo();
}
