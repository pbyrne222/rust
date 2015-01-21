// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct Stuff {
  a: int,
  b: f64
}

pub trait Trait {
    fn method(&self) -> Stuff;
}

#[no_mangle]
pub fn test(t: &Trait) -> int {
    t.method().a
}
