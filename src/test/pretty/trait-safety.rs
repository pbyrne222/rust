// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pp-exact

unsafe trait UnsafeTrait {
    fn foo(&self);
}

unsafe impl UnsafeTrait for int {
    fn foo(&self) { }
}

pub unsafe trait PubUnsafeTrait {
    fn foo(&self);
}

fn main() { }
