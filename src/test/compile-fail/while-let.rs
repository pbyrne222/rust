// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn macros() {
    macro_rules! foo{
        ($p:pat, $e:expr, $b:block) => {{
            while let $p = $e $b
        }}
    }
    macro_rules! bar{
        ($p:pat, $e:expr, $b:block) => {{
            foo!($p, $e, $b)
        }}
    }

    foo!(a, 1is, { //~ ERROR irrefutable while-let
        println!("irrefutable pattern");
    });
    bar!(a, 1is, { //~ ERROR irrefutable while-let
        println!("irrefutable pattern");
    });
}

pub fn main() {
    while let a = 1is { //~ ERROR irrefutable while-let
        println!("irrefutable pattern");
    }
}
