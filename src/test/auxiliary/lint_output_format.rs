// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="lint_output_format"]
#![crate_type = "lib"]
#![staged_api]

#[deprecated]
pub fn foo() -> uint {
    20
}

#[unstable]
pub fn bar() -> uint {
    40
}

#[unstable]
pub fn baz() -> uint {
    30
}
