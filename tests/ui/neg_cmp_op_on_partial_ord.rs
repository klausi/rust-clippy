// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This test case utilizes `f64` an easy example for `PartialOrd` only types
//! but the lint itself actually validates any expression where the left
//! operand implements `PartialOrd` but not `Ord`.

use std::cmp::Ordering;

#[warn(clippy::neg_cmp_op_on_partial_ord)]
fn main() {
    let a_value = 1.0;
    let another_value = 7.0;

    // --- Bad ---

    // Not Less but potentially Greater, Equal or Uncomparable.
    let _not_less = !(a_value < another_value);

    // Not Less or Equal but potentially Greater or Uncomparable.
    let _not_less_or_equal = !(a_value <= another_value);

    // Not Greater but potentially Less, Equal or Uncomparable.
    let _not_greater = !(a_value > another_value);

    // Not Greater or Equal but potentially Less or Uncomparable.
    let _not_greater_or_equal = !(a_value >= another_value);

    // --- Good ---

    let _not_less = match a_value.partial_cmp(&another_value) {
        None | Some(Ordering::Greater) | Some(Ordering::Equal) => true,
        _ => false,
    };
    let _not_less_or_equal = match a_value.partial_cmp(&another_value) {
        None | Some(Ordering::Greater) => true,
        _ => false,
    };
    let _not_greater = match a_value.partial_cmp(&another_value) {
        None | Some(Ordering::Less) | Some(Ordering::Equal) => true,
        _ => false,
    };
    let _not_greater_or_equal = match a_value.partial_cmp(&another_value) {
        None | Some(Ordering::Less) => true,
        _ => false,
    };

    // --- Should not trigger ---

    let _ = a_value < another_value;
    let _ = a_value <= another_value;
    let _ = a_value > another_value;
    let _ = a_value >= another_value;

    // --- regression tests ---

    // Issue 2856: False positive on assert!()
    //
    // The macro always negates the result of the given comparison in its
    // internal check which automatically triggered the lint. As it's an
    // external macro there was no chance to do anything about it which led
    // to a whitelisting of all external macros.
    assert!(a_value < another_value);
}
