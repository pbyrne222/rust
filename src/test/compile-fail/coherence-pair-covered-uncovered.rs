// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:coherence-lib.rs

extern crate "coherence-lib" as lib;
use lib::{Remote, Pair};

struct Local<T>(T);

impl<T,U> Remote for Pair<T,Local<U>> { }
//~^ ERROR type parameter `T` is not constrained

fn main() { }
