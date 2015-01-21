// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:private_trait_xc.rs

extern crate private_trait_xc;

struct Bar;

impl private_trait_xc::Foo for Bar {}
//~^ ERROR: trait `Foo` is private

fn main() {}

