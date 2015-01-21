// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: --cfg foo

#[cfg(foo)]
macro_rules! foo { () => (1i) }

#[cfg(not(foo))]
macro_rules! foo { () => (2i) }

pub fn main() {
    assert_eq!(foo!(), 1i);
}
