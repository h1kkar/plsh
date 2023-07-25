//! Little shell for you.
//!
//! This сrate is a full-fledged shell written on Rust, capable of working along
//! with `bash`.
//!
//! At the moment, it only works in Linux systems.
//! 
//! This crate is [on crates.io](https://crates.io/crates/shime).
//! 
//! # Examples
//! 
//! You really thought that I would write everything that can be launched? 
//! Of course not. Just run everything that is possible and enjoy.
//!
//! If you want to see internal functions, then look better the 
//! [main module][shime].

/// This is the main module that includes the entire functionality of the shell.
pub mod shime;

///This module is responsible for its own `shime` commands.
pub mod cmd;

#[doc(hidden)]
pub use shime::start;

#[doc(hidden)]
fn main() {
    start()
}