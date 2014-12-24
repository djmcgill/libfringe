#![feature(default_type_params, macro_rules)]
extern crate fn_box;

pub use context::Context;

#[macro_escape]
mod macros;

mod context;
mod stack;

mod arch;
mod platform;
