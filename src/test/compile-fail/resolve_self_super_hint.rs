// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod a {
    extern crate collections;
    use collections::HashMap;
//~^ ERROR unresolved import `collections::HashMap`. Did you mean `self::collections`?
    mod b {
        use collections::HashMap;
//~^ ERROR unresolved import `collections::HashMap`. Did you mean `a::collections`?
        mod c {
            use collections::HashMap;
//~^ ERROR unresolved import `collections::HashMap`. Did you mean `a::collections`?
            mod d {
                use collections::HashMap;
//~^ ERROR unresolved import `collections::HashMap`. Did you mean `a::collections`?
            }
        }
    }
}
