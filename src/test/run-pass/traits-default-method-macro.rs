// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait Foo {
    fn bar(&self) -> String {
        format!("test")
    }
}

enum Baz {
    Quux
}

impl Foo for Baz {
}

pub fn main() {
    let q = Baz::Quux;
    assert_eq!(q.bar(), "test".to_string());
}
