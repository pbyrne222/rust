// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(PartialEq, Show)]
enum Foo {
    Bar(int, int),
    Baz(f64, f64)
}

pub fn main() {
    let a = Foo::Bar(1, 2);
    let b = Foo::Bar(1, 2);
    assert_eq!(a, b);
    assert!(!(a != b));
    assert!(a.eq(&b));
    assert!(!a.ne(&b));
}
