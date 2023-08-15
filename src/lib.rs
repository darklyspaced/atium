#![allow(dead_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_const_for_fn,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::enum_glob_use,
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]

pub mod ast;
pub mod atium;
pub mod cli;
pub mod error;
pub mod impetuous;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod reporter;
pub mod token;
