// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

static mut destructions : int = 3;

pub fn foo() {
    #[unsafe_no_drop_flag]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
          unsafe { destructions -= 1 };
        }
    };

    let _x = [Foo, Foo, Foo];
}

pub fn main() {
  foo();
  assert!((unsafe { destructions } == 0));
}
