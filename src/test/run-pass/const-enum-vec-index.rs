// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum E { V1(int), V0 }
const C: &'static [E] = &[E::V0, E::V1(0xDEADBEE)];
static C0: E = C[0];
static C1: E = C[1];
const D: &'static [E; 2] = &[E::V0, E::V1(0xDEADBEE)];
static D0: E = C[0];
static D1: E = C[1];

pub fn main() {
    match C0 {
        E::V0 => (),
        _ => panic!()
    }
    match C1 {
        E::V1(n) => assert!(n == 0xDEADBEE),
        _ => panic!()
    }

    match D0 {
        E::V0 => (),
        _ => panic!()
    }
    match D1 {
        E::V1(n) => assert!(n == 0xDEADBEE),
        _ => panic!()
    }
}
