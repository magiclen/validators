#![cfg(all(feature = "length", feature = "derive"))]

extern crate alloc;

use alloc::collections::BTreeSet;

use validators::prelude::*;

#[derive(Validator)]
#[validator(length(min = 1))]
pub struct NonEmptyVec(pub Vec<u8>);

#[derive(Validator)]
#[validator(length(min = 1, max = 100))]
pub struct Set(pub BTreeSet<isize>);

#[test]
fn basic() {
    assert!(NonEmptyVec::parse_collection(vec![8]).is_ok());
    assert!(NonEmptyVec::parse_collection(vec![]).is_err());
    assert!(Set::parse_collection(BTreeSet::new()).is_err());
}
