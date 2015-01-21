// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo { a: isize, b: isize }
struct Bar { x: isize }

static bar: Bar = Bar { x: 5 };
static foo: Foo = Foo { a: 2, ..bar }; //~  ERROR mismatched types
                                       //~| expected `Foo`
                                       //~| found `Bar`
                                       //~| expected struct `Foo`
                                       //~| found struct `Bar`
static foo_i: Foo = Foo { a: 2, ..4 }; //~  ERROR mismatched types
                                       //~| expected `Foo`
                                       //~| found `_`
                                       //~| expected struct `Foo`
                                       //~| found integral variable

fn main() {
    let b = Bar { x: 5 };
    let f = Foo { a: 2, ..b }; //~  ERROR mismatched types
                               //~| expected `Foo`
                               //~| found `Bar`
                               //~| expected struct `Foo`
                               //~| found struct `Bar`
    let f_i = Foo { a: 2, ..4 }; //~  ERROR mismatched types
                                 //~| expected `Foo`
                                 //~| found `_`
                                 //~| expected struct `Foo`
                                 //~| found integral variable
}
