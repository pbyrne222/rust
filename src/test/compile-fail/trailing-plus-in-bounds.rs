// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::Show;

fn main() {
    let x: Box<Show+> = box 3is as Box<Show+>;
    //~^ ERROR at least one type parameter bound must be specified
    //~^^ ERROR at least one type parameter bound must be specified
}

