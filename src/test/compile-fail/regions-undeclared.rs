// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

static c_x: &'blk isize = &22; //~ ERROR use of undeclared lifetime name `'blk`

enum EnumDecl {
    Foo(&'a isize), //~ ERROR use of undeclared lifetime name `'a`
    Bar(&'a isize), //~ ERROR use of undeclared lifetime name `'a`
}

fn fnDecl(x: &'a isize, //~ ERROR use of undeclared lifetime name `'a`
          y: &'a isize) //~ ERROR use of undeclared lifetime name `'a`
{}

fn main() {
}
