// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct S { f0: String, f1: int }

pub fn main() {
    let s = "Hello, world!".to_string();
    let _s = S {
        f0: s.to_string(),
        ..S {
            f0: s,
            f1: 23
        }
    };
}
