// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:crateresolve4a-1.rs
// aux-build:crateresolve4a-2.rs
#![crate_name="crateresolve4b#0.2"]
#![crate_type = "lib"]

extern crate "crateresolve4a#0.1" as crateresolve4a;

pub fn g() -> int { crateresolve4a::f() }
