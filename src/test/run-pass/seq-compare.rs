// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


pub fn main() {
    assert!(("hello".to_string() < "hellr".to_string()));
    assert!(("hello ".to_string() > "hello".to_string()));
    assert!(("hello".to_string() != "there".to_string()));
    assert!((vec!(1i, 2, 3, 4) > vec!(1, 2, 3)));
    assert!((vec!(1i, 2, 3) < vec!(1, 2, 3, 4)));
    assert!((vec!(1i, 2, 4, 4) > vec!(1, 2, 3, 4)));
    assert!((vec!(1i, 2, 3, 4) < vec!(1, 2, 4, 4)));
    assert!((vec!(1i, 2, 3) <= vec!(1, 2, 3)));
    assert!((vec!(1i, 2, 3) <= vec!(1, 2, 3, 3)));
    assert!((vec!(1i, 2, 3, 4) > vec!(1, 2, 3)));
    assert_eq!(vec!(1i, 2, 3), vec!(1, 2, 3));
    assert!((vec!(1i, 2, 3) != vec!(1, 1, 3)));
}
