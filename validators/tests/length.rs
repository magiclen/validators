#![cfg(all(feature = "test", feature = "derive", feature = "length"))]

use std::collections::BTreeSet;

use validators::prelude::*;

#[derive(Validator)]
#[validator(length(min = 1))]
pub struct NonEmptyVec(pub Vec<u8>);

#[derive(Validator)]
#[validator(length(min = 1, max = 100))]
pub struct Set(pub BTreeSet<isize>);

#[test]
fn basic() {
    #[derive(Validator)]
    #[validator(length(min = 3, max = 3))]
    struct FixedLength(Vec<u8>);

    assert!(NonEmptyVec::parse_collection(vec![8]).is_ok());
    assert!(NonEmptyVec::parse_collection(vec![]).is_err());
    assert!(Set::parse_collection(BTreeSet::new()).is_err());
    assert_eq!(vec![1, 2, 3], FixedLength::parse_collection(vec![1, 2, 3]).unwrap().0);
}

#[test]
fn other_collections() {
    use std::collections::VecDeque;

    #[derive(Validator)]
    #[validator(length(min = 1, max = 2))]
    struct Queue(VecDeque<u8>);

    assert!(Queue::parse_collection(VecDeque::from([1, 2])).is_ok());
    assert!(Queue::parse_collection(VecDeque::new()).is_err());

    // A hash set with another hasher, such as `FxHashSet`, also works.
    #[cfg(feature = "std")]
    {
        use std::{
            collections::HashSet,
            hash::{BuildHasherDefault, DefaultHasher},
        };

        type CustomHashSet = HashSet<u8, BuildHasherDefault<DefaultHasher>>;

        #[derive(Validator)]
        #[validator(length(min = 1, max = 2))]
        struct CustomSet(CustomHashSet);

        assert!(CustomSet::parse_collection(CustomHashSet::from_iter([1, 2])).is_ok());
        assert!(CustomSet::parse_collection(CustomHashSet::from_iter([1, 2, 3])).is_err());
    }
}

#[cfg(feature = "serde_json")]
#[test]
fn wrapped_result() {
    use validators::{Result, errors::LengthError, serde_json};

    #[derive(Debug, Validator)]
    #[validator(length(min = 1))]
    struct NonEmpty(Vec<u8>);

    let empty: Result<NonEmpty, LengthError, Vec<u8>> = serde_json::from_str("[]").unwrap();

    assert!(empty.is_err());
    assert_eq!("Err(TooSmall)", format!("{empty:?}"));
}
