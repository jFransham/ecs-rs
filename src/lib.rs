#![feature(reflect_marker)]
#![feature(associated_type_defaults)]

extern crate dynamic;

#[macro_use]
mod macros;
mod heterogenous_set;

pub mod entity;
pub mod components;
pub mod system;
pub mod either;
