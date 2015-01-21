// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum ast<'a> {
    num(uint),
    add(&'a ast<'a>, &'a ast<'a>)
}

fn mk_add_ok<'r>(x: &'r ast<'r>, y: &'r ast<'r>) -> ast<'r> {
    ast::add(x, y)
}

pub fn main() {
}
