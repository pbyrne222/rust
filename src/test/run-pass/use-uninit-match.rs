// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



fn foo<T>(o: myoption<T>) -> int {
    let mut x: int = 5;
    match o {
        myoption::none::<T> => { }
        myoption::some::<T>(_t) => { x += 1; }
    }
    return x;
}

enum myoption<T> { none, some(T), }

pub fn main() { println!("{}", 5i); }
