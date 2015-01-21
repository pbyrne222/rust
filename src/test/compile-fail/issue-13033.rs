// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Foo {
    fn bar(&mut self, other: &mut Foo);
}

struct Baz;

impl Foo for Baz {
    fn bar(&mut self, other: &Foo) {}
    //~^ ERROR method `bar` has an incompatible type for trait: values differ in mutability [E0053]
}

fn main() {}
