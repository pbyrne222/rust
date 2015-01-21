// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that non-constant exprs do fail as count in fixed length vec type

fn main() {
    fn bar(n: isize) {
        let _x: [isize; n];
        //~^ ERROR expected constant expr for array length: non-constant path in constant expr
    }
}
