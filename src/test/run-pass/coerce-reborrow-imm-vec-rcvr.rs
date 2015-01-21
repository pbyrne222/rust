// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


fn bar(v: &mut [uint]) -> Vec<uint> {
    v.to_vec()
}

fn bip(v: &[uint]) -> Vec<uint> {
    v.to_vec()
}

pub fn main() {
    let mut the_vec = vec!(1u, 2, 3, 100);
    assert_eq!(the_vec.clone(), bar(the_vec.as_mut_slice()));
    assert_eq!(the_vec.clone(), bip(the_vec.as_slice()));
}
