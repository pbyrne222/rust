// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pp-exact

// Check that `let x: _ = 0;` does not print as `let x = 0;`.

fn main() {
    let x: _ = 0;

    let _ = x;
}
