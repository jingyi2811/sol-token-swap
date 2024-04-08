//! Fixed length string types
use {
    arraystring::{typenum::U64, ArrayString},
};

/// Fixed size array to store UTF-8 strings on blockchain.
pub type ArrayString64 = ArrayString<U64>;