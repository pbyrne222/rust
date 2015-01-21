// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link()] //~ ERROR: specified without `name =
#[link(name = "")] //~ ERROR: with empty name
#[link(name = "foo")]
#[link(name = "foo", kind = "bar")] //~ ERROR: unknown kind
extern {}

fn main() {}
