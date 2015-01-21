// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    let x = true;
    if x { let mut i = 10i; while i > 0 { i -= 1; } }
    match x { true => { println!("right"); } false => { println!("wrong"); } }
}
