// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test method calls with self as an argument cannot subvert type checking.

struct Foo;

impl Foo {
    fn bar(&self) {}
}

fn main() {
    let x = Foo;
    Foo::bar(x); //~  ERROR mismatched types
                 //~| expected `&Foo`
                 //~| found `Foo`
                 //~| expected &-ptr
                 //~| found struct `Foo`
    Foo::bar(&&x); //~  ERROR mismatched types
                   //~| expected `&Foo`
                   //~| found `&&Foo`
                   //~| expected struct `Foo`
                   //~| found &-ptr
    Foo::bar(&42is); //~  ERROR mismatched types
                     //~| expected `&Foo`
                     //~| found `&isize`
                     //~| expected struct `Foo`
                     //~| found isize
}
