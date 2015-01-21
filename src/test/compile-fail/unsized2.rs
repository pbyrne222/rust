// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test syntax checks for `type` keyword.

fn f<X>() {}

pub fn main() {
    f<type>();
    //~^ ERROR expected identifier, found keyword `type`
    //~^^ ERROR: Chained comparison operators require parentheses
    //~^^^ HELP: Use ::< instead of < if you meant to specify type arguments.
}
