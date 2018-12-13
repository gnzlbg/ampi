//! Representation and validity compatibility

use crate::{mem, Datatype};

/// `Self` is a suitable MPI type to hold an `I`.
///
/// Suitable means that `Self` representation, validity, and safety invariants
/// hold for all bit-patterns of `I`. This implies that `Self` size and
/// alignment requirements are valid for `I`.
///
/// It also means that the MPI Datatypes of `Self` and `I` are equal.
pub unsafe trait Compatible<I: Datatype>: Datatype {}

/// For all types `T`, `T` is suitable for any `T`
unsafe impl<T: Datatype> Compatible<T> for T {}

/// `MaybeUninit<T>` is suitable to hold any `T`
unsafe impl<T: Datatype> Compatible<T> for mem::MaybeUninit<T> where
    mem::MaybeUninit<T>: Datatype
{
}
