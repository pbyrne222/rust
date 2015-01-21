// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test unboxed closure sugar used in object types.

#![allow(dead_code)]
#![feature(unboxed_closures)]

struct Foo<T,U> {
    t: T, u: U
}

trait Getter<A,R> {
    fn get(&self, arg: A) -> R;
}

struct Identity;
impl<X> Getter<X,X> for Identity {
    fn get(&self, arg: X) -> X {
        arg
    }
}

fn main() {
    let x: &Getter(int) -> (int,) = &Identity;
    let (y,) = x.get((22,));
    assert_eq!(y, 22);
}
