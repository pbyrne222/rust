// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test for a subtle failure computing kinds of cyclic types, in which
// temporary kinds wound up being stored in a cache and used later.
// See middle::ty::type_contents() for more information.


struct List { key: isize, next: Option<Box<List>> }

fn foo(node: Box<List>) -> isize {
    let r = match node.next {
        Some(right) => consume(right),
        None => 0
    };
    consume(node) + r //~ ERROR use of partially moved value: `node`
}

fn consume(v: Box<List>) -> isize {
    v.key
}

fn main() {}
