// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! mypat {
    () => (
        Some('y')
    )
}

macro_rules! char_x {
    () => (
        'x'
    )
}

macro_rules! some {
    ($x:pat) => (
        Some($x)
    )
}

macro_rules! indirect {
    () => (
        some!(char_x!())
    )
}

macro_rules! ident_pat {
    ($x:ident) => (
        $x
    )
}

fn f(c: Option<char>) -> uint {
    match c {
        Some('x') => 1,
        mypat!() => 2,
        _ => 3,
    }
}

pub fn main() {
    assert_eq!(1u, f(Some('x')));
    assert_eq!(2u, f(Some('y')));
    assert_eq!(3u, f(None));

    assert_eq!(1i, match Some('x') {
        Some(char_x!()) => 1i,
        _ => 2i,
    });

    assert_eq!(1i, match Some('x') {
        some!(char_x!()) => 1i,
        _ => 2i,
    });

    assert_eq!(1i, match Some('x') {
        indirect!() => 1i,
        _ => 2i,
    });

    assert_eq!(3i, {
        let ident_pat!(x) = 2i;
        x+1i
    });
}
