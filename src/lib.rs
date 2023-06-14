#![allow(dead_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_const_for_fn,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]
pub mod ast;
pub mod atium;
pub mod parser;
pub mod scanner;
pub mod token;
