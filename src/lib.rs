#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Iron Sea - Index
//!
//! This repository contains the traits definitions for the Iron Sea
//! database toolkit indices.
//!
//! ## Iron Sea: Database Toolkit
//! **Iron Sea** provides a set of database engine bricks, which can be
//! combined and applied on arbitrary data structures.
//!
//! Unlike a traditional database, it does not assume a specific
//! physical structure for the tables nor the records, but relies on the
//! developer to provide a set of extractor functions which are used by
//! the specific indices provided.
//!
//! This enables the index implementations to be agnostic from the
//! underlying data structure, and re-used.
//!
//!

/// Record behavior used by Indexed implementations.
///
/// This trait provides common methods used by index implementations to
/// retrieve information about a single record. This is provided by the
/// users of indices, for each of their `struct` they wish to index.
///
/// Multiple implementation can be provided, as long as their types are
/// different.
///
// TODO: Add more complex scenarii where multiple implementations with
//       the same types are necessary, for example returning either a or
//       b.
///
/// # Examples
///
/// ```
/// use ironsea_index::Record;
///
/// #[derive(Clone, Debug)]
/// pub struct MyPair {
///     a: i64,
///     b: i64,
/// }
///
/// impl Record<String> for MyPair {
///    fn key(&self) -> String {
///        format!("{}", self.a)
///    }
/// }
///
/// impl Record<i64> for MyPair {
///    fn key(&self) -> i64 {
///        self.a
///    }
/// }
///
/// fn main() {
///
///    let table = vec![MyPair{ a: 10, b:34}, MyPair{ a: 1, b:56}, MyPair{ a: 2, b:23}];
///
///    // Example without using an actual index crate, we will simply use
///    // the Record<K> trait to sort the array of pairs.
///    let mut lex_sort = table.clone();
///    lex_sort.sort_unstable_by_key(|e| {let k: String = e.key(); k});
///
///    let mut num_sort = table.clone();
///    num_sort.sort_unstable_by_key(|e| {let k: i64 = e.key(); k});
///
///    assert_eq!(format!("unsorted {:?}", table),
///         "unsorted [MyPair { a: 10, b: 34 }, MyPair { a: 1, b: 56 }, MyPair { a: 2, b: 23 }]");
///    assert_eq!(format!("lex sort {:?}", lex_sort),
///         "lex sort [MyPair { a: 1, b: 56 }, MyPair { a: 10, b: 34 }, MyPair { a: 2, b: 23 }]");
///    assert_eq!(format!("num sort {:?}", num_sort),
///         "num sort [MyPair { a: 1, b: 56 }, MyPair { a: 2, b: 23 }, MyPair { a: 10, b: 34 }]");
/// }
/// ```
pub trait Record<K> {
    /// Extract the key from the record.
    fn key(&self) -> K;
}

/// Record behavior used by IndexedDestructured implementations.
///
/// RecordFields is used by indices which de-structure records into two
/// components, the key and the fields associated to that unique key.
/// This is provided by the users of indices, for each of their `struct`
/// they wish to index.
pub trait RecordFields<F> {
    /// Extract the fields of the record
    fn fields(&self) -> F;
}

/// Methods provided by indices.
///
/// This kind of indices can work on references to the original vector
/// or take ownership of the records, based on the type given for the
/// records.
///
///  * `R`: Type of the records
///  * `K`: Type of the keys

// Generic types are not sorted alphabetically, to match next trait
// semantic order
pub trait Indexed<R, K> {
    /// Retrieve all records matching the key.
    fn find(&self, key: &K) -> Vec<&R>;

    /// Retrieve all records matching in the key range defined by
    /// `start` and `end`.
    ///
    /// * `start` is included
    // TODO: TBC for `end`
    fn find_range(&self, start: &K, end: &K) -> Vec<&R>;
}

/// Methods provided by destructuring indices.
///
/// This kind of indices store inside the index both keys and values,
/// meaning the original records can be freed.
///
///  * `F`: Type of the struct containing the remaining fields
///  * `K`: Type of the keys
pub trait IndexedDestructured<F, K> {
    /// Retrieve all records matching the key.
    fn find(&self, key: &K) -> Vec<&F>;

    /// Retrieve all records matching in the key range defined by
    /// `start` and `end`.
    ///
    /// * `start` is included
    // TODO: TBC for `end`
    fn find_range(&self, start: &K, end: &K) -> Vec<(K, &F)>;
}
