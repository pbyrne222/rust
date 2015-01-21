// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Checks that the Fn trait hierarchy rules do not permit
// Fn to be used where FnMut is implemented.

#![feature(unboxed_closures)]
#![feature(overloaded_calls)]

use std::ops::{Fn,FnMut,FnOnce};

struct S;

impl FnMut<(isize,),isize> for S {
    extern "rust-call" fn call_mut(&mut self, (x,): (isize,)) -> isize {
        x * x
    }
}

fn call_it<F:Fn(isize)->isize>(f: &F, x: isize) -> isize {
    f.call((x,))
}

fn main() {
    let x = call_it(&S, 22); //~ ERROR not implemented
}

