// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_imports)]

extern crate issue_2316_a;

pub mod cloth {
    use issue_2316_a::*;

    pub enum fabric {
        gingham, flannel, calico
    }
}
