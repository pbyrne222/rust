// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test lifetimes are linked properly when we take reference
// to interior.

struct Foo(int);
pub fn main() {
    // Here the lifetime of the `&` should be at least the
    // block, since a ref binding is created to the interior.
    let &Foo(ref _x) = &Foo(3);
}
