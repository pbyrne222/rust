// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Get {
    type Value;
    fn get(&self) -> <Self as Get>::Value;
}

struct Struct {
    x: isize,
}

impl Struct {
    fn uhoh<T>(foo: <T as Get>::Value) {}
    //~^ ERROR the trait `Get` is not implemented for the type `T`
}

fn main() {
}
