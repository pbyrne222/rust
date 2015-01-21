// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    let a: String = "this \
is a test".to_string();
    let b: String =
        "this \
              is \
              another \
              test".to_string();
    assert_eq!(a, "this is a test".to_string());
    assert_eq!(b, "this is another test".to_string());
}
