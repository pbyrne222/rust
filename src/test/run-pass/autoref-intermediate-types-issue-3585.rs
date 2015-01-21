// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(unknown_features)]
#![feature(box_syntax)]

trait Foo {
    fn foo(&self) -> String;
}

impl<T:Foo> Foo for Box<T> {
    fn foo(&self) -> String {
        format!("box {}", (**self).foo())
    }
}

impl Foo for uint {
    fn foo(&self) -> String {
        format!("{}", *self)
    }
}

pub fn main() {
    let x = box 3u;
    assert_eq!(x.foo(), "box 3".to_string());
}
