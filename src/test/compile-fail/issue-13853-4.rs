// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct AutoBuilder<'a> {
    context: &'a isize
}

impl<'a> Drop for AutoBuilder<'a> {
    //~^ ERROR cannot implement a destructor on a structure with type parameters
    fn drop(&mut self) {
    }
}

fn main() {}
