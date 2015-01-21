// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn test() {
    let v: isize;
    loop {
        v = 1; //~ ERROR re-assignment of immutable variable
        //~^ NOTE prior assignment occurs here
        v.clone(); // just to prevent liveness warnings
    }
}

fn main() {
}
