// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax)]

enum Foo {
    Foo1(Box<u32>, Box<u32>),
    Foo2(Box<u32>),
    Foo3,
}

fn blah() {
    let f = &Foo::Foo1(box 1u32, box 2u32);
    match *f {             //~ ERROR cannot move out of
        Foo::Foo1(num1,         //~ NOTE attempting to move value to here
                  num2) => (),  //~ NOTE and here
        Foo::Foo2(num) => (),   //~ NOTE and here
        Foo::Foo3 => ()
    }
}

struct S {
    f: String,
    g: String
}
impl Drop for S {
    fn drop(&mut self) { println!("{}", self.f); }
}

fn move_in_match() {
    match (S {f: "foo".to_string(), g: "bar".to_string()}) {
        S {         //~ ERROR cannot move out of type `S`, which defines the `Drop` trait
            f: _s,  //~ NOTE attempting to move value to here
            g: _t   //~ NOTE and here
        } => {}
    }
}

// from issue-8064
struct A {
    a: Box<isize>,
}

fn free<T>(_: T) {}

fn blah2() {
    let a = &A { a: box 1 };
    match a.a {           //~ ERROR cannot move out of
        n => {            //~ NOTE attempting to move value to here
            free(n)
        }
    }
    free(a)
}

fn main() {}
