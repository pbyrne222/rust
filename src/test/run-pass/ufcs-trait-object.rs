// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that when you use ufcs form to invoke a trait method (on a
// trait object) everything works fine.

trait Foo {
    fn test(&self) -> i32;
}

impl Foo for i32 {
    fn test(&self) -> i32 { *self }
}

fn main() {
    let a: &Foo = &22_i32;
    assert_eq!(Foo::test(a), 22);
}
