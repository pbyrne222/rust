// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:capacity overflow

use std::collections::hash_map::HashMap;
use std::uint;
use std::mem::size_of;

fn main() {
    let threshold = uint::MAX / size_of::<(u64, u64, u64)>();
    let mut h = HashMap::<u64, u64>::with_capacity(threshold + 100);
    h.insert(0, 0);
}
