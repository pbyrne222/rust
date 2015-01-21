// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -F unstable

#![staged_api]
#[allow(unstable)] //~ ERROR allow(unstable) overruled by outer forbid(unstable)
fn main() {
}
