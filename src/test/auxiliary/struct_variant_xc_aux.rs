// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="struct_variant_xc_aux"]
#![crate_type = "lib"]

pub enum Enum {
    Variant(u8),
    StructVariant { arg: u8 }
}
