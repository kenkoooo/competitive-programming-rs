//! # The Competitive Programming Library in Rust
//! The Competitive Programming Library in Rust is a set of snippets,
//! which can be used in programming contests.

#![feature(test)]

extern crate test;

extern crate num_traits;

/// Algorithms related to graph theory.
pub mod graph;

/// Structures that can efficiently operate data.
pub mod data_structure;

/// Functions to test this library with contest-style IO files.
pub mod test_helper;

/// Mathematical algorithms.
pub mod math;

/// Geometry algorithms.
pub mod geometry;
