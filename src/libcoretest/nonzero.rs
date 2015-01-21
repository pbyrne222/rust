// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::nonzero::NonZero;
use core::option::Option;
use core::option::Option::{Some, None};
use std::mem::size_of;

#[test]
fn test_create_nonzero_instance() {
    let _a = unsafe {
        NonZero::new(21i)
    };
}

#[test]
fn test_size_nonzero_in_option() {
    assert_eq!(size_of::<NonZero<u32>>(), size_of::<Option<NonZero<u32>>>());
}

#[test]
fn test_match_on_nonzero_option() {
    let a = Some(unsafe {
        NonZero::new(42i)
    });
    match a {
        Some(val) => assert_eq!(*val, 42),
        None => panic!("unexpected None while matching on Some(NonZero(_))")
    }

    match unsafe { Some(NonZero::new(43i)) } {
        Some(val) => assert_eq!(*val, 43),
        None => panic!("unexpected None while matching on Some(NonZero(_))")
    }
}

#[test]
fn test_match_option_empty_vec() {
    let a: Option<Vec<int>> = Some(vec![]);
    match a {
        None => panic!("unexpected None while matching on Some(vec![])"),
        _ => {}
    }
}

#[test]
fn test_match_option_vec() {
    let a = Some(vec![1i, 2, 3, 4]);
    match a {
        Some(v) => assert_eq!(v, vec![1i, 2, 3, 4]),
        None => panic!("unexpected None while matching on Some(vec![1, 2, 3, 4])")
    }
}

#[test]
fn test_match_option_rc() {
    use std::rc::Rc;

    let five = Rc::new(5i);
    match Some(five) {
        Some(r) => assert_eq!(*r, 5i),
        None => panic!("unexpected None while matching on Some(Rc::new(5))")
    }
}

#[test]
fn test_match_option_arc() {
    use std::sync::Arc;

    let five = Arc::new(5i);
    match Some(five) {
        Some(a) => assert_eq!(*a, 5i),
        None => panic!("unexpected None while matching on Some(Arc::new(5))")
    }
}

#[test]
fn test_match_option_empty_string() {
    let a = Some(String::new());
    match a {
        None => panic!("unexpected None while matching on Some(String::new())"),
        _ => {}
    }
}

#[test]
fn test_match_option_string() {
    let five = "Five".to_string();
    match Some(five) {
        Some(s) => assert_eq!(s, "Five"),
        None => panic!("unexpected None while matching on Some(String { ... })")
    }
}
