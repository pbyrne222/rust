// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn test() {
    let w: &mut [isize];
    w[5] = 0; //~ ERROR use of possibly uninitialized variable: `w`

    let mut w: &mut [isize];
    w[5] = 0; //~ ERROR use of possibly uninitialized variable: `w`
}

fn main() { test(); }
