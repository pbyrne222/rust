// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate getopts;

use getopts::{optopt, getopts};

pub fn main() {
    let args = Vec::new();
    let opts = vec!(optopt("b", "", "something", "SMTHNG"));

    match getopts(args.as_slice(), opts.as_slice()) {
        Ok(ref m)  =>
            assert!(!m.opt_present("b")),
        Err(ref f) => panic!("{}", *f)
    };

}
