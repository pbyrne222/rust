// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Issue #8624. Test for reborrowing with 3 levels, not just two.

fn copy_borrowed_ptr<'a, 'b, 'c>(p: &'a mut &'b mut &'c mut isize) -> &'b mut isize {
    &mut ***p //~ ERROR cannot infer
}

fn main() {
}
