// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

const X: &'static str = "12345";

fn test(s: String) -> bool {
    match s.as_slice() {
        X => true,
        _ => false
    }
}

fn main() {
    assert!(test("12345".to_string()));
}
