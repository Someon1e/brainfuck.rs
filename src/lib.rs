#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]

pub mod compile;
pub mod lexer;
pub mod interpreter;
pub mod into_rust;

/// Initial memory capacity, however memory will resize when full.
pub const INITIAL_MEMORY_CAPACITY: usize = 32;

/// When out of memory, increase size by this many bytes.
pub const MEMORY_RESIZE_AMOUNT: usize = 16;

