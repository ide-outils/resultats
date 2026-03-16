#![feature(try_trait_v2)]
//! # Résultats
//! The main goal of this crate is to provide a facilities to handle multiple errors.
//! This is mainly usefull for a rust bin to act like a script that run multiple tasks.
//!
//! ## Design
//! It basiacally implements the traits `Try`, `FromResidual`, `FromIterator`, `Termination` for
//! `resultats::Result<T>`.
//!
//! ## Example
//! The basic usage is from an iterator that return either a `std::result::Result` ou a
//! `resultats::Result` :
//! ```
//! let iter = ["et", "1", "et", "2", "et", "3", "0"]
//! let parsed = iter.map(str::parse).collect::<Result<Vec<usize>>>()?;
//! ```
pub mod ast;
pub mod format_error;
pub mod result;
pub mod source_code;

pub use result::{
    Error, Errors,
    Result::{self, Err, Ok},
};
