// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



mod m1 {
    pub enum foo { foo1, foo2, }
}

fn bar(x: m1::foo) { match x { m1::foo::foo1 => { } m1::foo::foo2 => { } } }

pub fn main() { }
