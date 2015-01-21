// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Show)]
struct foo {
  i: isize,
}

impl Drop for foo {
    fn drop(&mut self) {}
}

fn foo(i:isize) -> foo {
    foo {
        i: i
    }
}

fn main() {
    let x = foo(10);
    let _y = x.clone();
    //~^ ERROR does not implement any method in scope
    println!("{:?}", x);
}
