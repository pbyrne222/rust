// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

static a: &'static str = "foo";
static b: *const u8 = a as *const u8;
//~^ ERROR mismatched types
//~| expected *const u8
//~| found &'static str
//~| expected u8
//~| found str
static c: *const u8 = &a as *const u8;
//~^ ERROR mismatched types
//~| expected *const u8
//~| found &&'static str
//~| expected u8
//~| found &-ptr

fn main() {
}
