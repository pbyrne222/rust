// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
extern {
    fn CFRunLoopGetTypeID() -> libc::c_ulong;
}

#[cfg(target_os = "macos")]
pub fn main() {
    unsafe { CFRunLoopGetTypeID(); }
}

#[cfg(not(target_os = "macos"))]
pub fn main() {}
