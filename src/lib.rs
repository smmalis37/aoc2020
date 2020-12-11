#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::enum_glob_use,
    clippy::match_on_vec_items,
    clippy::missing_const_for_fn,
    clippy::needless_range_loop
)]

pub mod day_solver;
pub mod days;
pub(crate) mod util;
