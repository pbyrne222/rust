// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test sized-ness checking in substitution in impls.

// impl - struct

struct S5<Y>;

impl<X: ?Sized> S5<X> { //~ ERROR not implemented
}

fn main() { }
