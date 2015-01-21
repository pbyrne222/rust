// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

extern {
    #[link_name = "malloc"]
    fn malloc1(len: libc::c_int) -> *const libc::c_void;
    #[link_name = "malloc"]
    fn malloc2(len: libc::c_int, foo: libc::c_int) -> *const libc::c_void;
}

pub fn main () {}
