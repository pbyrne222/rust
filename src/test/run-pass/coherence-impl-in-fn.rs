// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    enum x { foo }
    impl Copy for x {}
    impl ::std::cmp::PartialEq for x {
        fn eq(&self, other: &x) -> bool {
            (*self) as int == (*other) as int
        }
        fn ne(&self, other: &x) -> bool { !(*self).eq(other) }
    }
}
