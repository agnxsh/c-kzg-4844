#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// This `extern crate` invocation tells `rustc` that we actually need the symbols from `blst`.
// Without it, the compiler won't link to `blst` when compiling this crate.
// See: https://kornel.ski/rust-sys-crate#linking
extern crate blst;

mod bindings;

// Expose relevant types with idiomatic names.
pub use bindings::{
    KZGCommitment as KzgCommitment, KZGProof as KzgProof, KZGSettings as KzgSettings,
    C_KZG_RET as CkzgError,
};
// Expose the constants.
pub use bindings::{
    BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_FIELD_ELEMENT, BYTES_PER_G1_POINT,
    BYTES_PER_G2_POINT, BYTES_PER_PROOF, CELLS_PER_EXT_BLOB, FIELD_ELEMENTS_PER_BLOB,
    FIELD_ELEMENTS_PER_CELL, FIELD_ELEMENTS_PER_EXT_BLOB,
};
// Expose the remaining relevant types.
pub use bindings::{Blob, Bytes32, Bytes48, Cell, Error};
