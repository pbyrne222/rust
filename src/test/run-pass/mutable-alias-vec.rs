// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn grow(v: &mut Vec<int> ) {
    v.push(1);
}

pub fn main() {
    let mut v: Vec<int> = Vec::new();
    grow(&mut v);
    grow(&mut v);
    grow(&mut v);
    let len = v.len();
    println!("{}", len);
    assert_eq!(len, 3 as uint);
}
