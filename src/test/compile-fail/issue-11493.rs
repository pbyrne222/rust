// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This file must never have a trailing newline

fn main() {
    let x = Some(3is);
    let y = x.as_ref().unwrap_or(&5is); //~ ERROR: borrowed value does not live long enough
}
