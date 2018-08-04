#![no_std]

mod std {
    pub use core::fmt;
    pub use core::str;
}

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod style;
