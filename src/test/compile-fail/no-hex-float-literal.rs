// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:hexadecimal float literal is not supported

fn main() {
    0xABC.Df;
    0x567.89;
    0xDEAD.BEEFp-2f;
}
