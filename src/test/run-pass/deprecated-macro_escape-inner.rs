// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty

mod foo {
    #![macro_escape] //~ WARNING macro_escape is a deprecated synonym for macro_use
    //~^ HELP consider an outer attribute
}

fn main() {
}
