// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Regression test for #20676. Error was that we didn't support
// UFCS-style calls to a method in `Trait` where `Self` was bound to a
// trait object of type `Trait`. See also `ufcs-trait-object.rs`.

use std::fmt;

fn main() {
    let a: &fmt::Show = &1_i32;
    format!("{:?}", a);
}
