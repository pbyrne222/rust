// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait foo {
    fn bar(&self, x: usize) -> Self;
}
impl foo for isize {
    fn bar(&self) -> isize {
        //~^ ERROR method `bar` has 1 parameter but the declaration in trait `foo::bar` has 2
        *self
    }
}

fn main() {
}
