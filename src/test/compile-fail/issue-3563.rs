// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait A {
    fn a(&self) {
        |&:| self.b()
        //~^ ERROR type `&Self` does not implement any method in scope named `b`
        //~| ERROR mismatched types
        //~| expected `()`
        //~| found closure
        //~| expected ()
        //~| found closure
    }
}
fn main() {}
