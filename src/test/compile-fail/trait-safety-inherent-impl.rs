// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that inherent impls cannot be unsafe.

struct SomeStruct;

unsafe impl SomeStruct { //~ ERROR inherent impls cannot be declared as unsafe
    fn foo(self) { }
}

fn main() { }
