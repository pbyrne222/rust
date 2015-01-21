// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;
fn dispose(_x: Arc<bool>) { }

pub fn main() {
    let p = Arc::new(true);
    let x = Some(p);
    match x {
        Some(z) => { dispose(z); },
        None => panic!()
    }
}
