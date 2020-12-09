#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::enum_glob_use,
    clippy::match_on_vec_items,
    clippy::missing_const_for_fn
)]

pub mod day_solver;
pub mod days;
pub(crate) mod util;

#[macro_use]
extern crate derive_builder;
