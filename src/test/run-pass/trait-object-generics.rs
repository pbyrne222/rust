// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// test for #8664

#![allow(unknown_features)]
#![feature(box_syntax)]

pub trait Trait2<A> {
    fn doit(&self);
}

pub struct Impl<A1, A2, A3> {
    /*
     * With A2 we get the ICE:
     * task <unnamed> failed at 'index out of bounds: the len is 1 but the index is 1',
     * src/librustc/middle/subst.rs:58
     */
    t: Box<Trait2<A2>+'static>
}

impl<A1, A2, A3> Impl<A1, A2, A3> {
    pub fn step(&self) {
        self.t.doit()
    }
}

// test for #8601

enum Type<T> { Constant }

trait Trait<K,V> {
    fn method(&self,Type<(K,V)>) -> int;
}

impl<V> Trait<u8,V> for () {
    fn method(&self, _x: Type<(u8,V)>) -> int { 0 }
}

pub fn main() {
    let a = box() () as Box<Trait<u8, u8>>;
    assert_eq!(a.method(Type::Constant), 0);
}
