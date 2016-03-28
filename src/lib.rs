#![feature(reflect_marker)]
#![feature(get_type_id)]

#[macro_use]
mod macros;
mod heterogenous_set;

pub mod entity;
pub mod components;
pub mod system;
pub mod either;
