//! Brainfuck executor in Rust

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]

/// Compile Brainfuck into tokens.
pub mod lexer;

/// Compile tokens into instructions.
pub mod compile;

/// Interpret instructions.
pub mod interpreter;

/// Compile instructions to Rust.
pub mod into_rust;

/// Compile instructions to C.
pub mod into_c;

/// Initial memory capacity, however memory will resize when full.
pub const INITIAL_MEMORY_CAPACITY: usize = 32;

/// When out of memory, increase size by this many bytes.
pub const MEMORY_RESIZE_AMOUNT: usize = 16;
