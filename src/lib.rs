#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::enum_glob_use,
    clippy::match_on_vec_items,
    clippy::missing_const_for_fn,
    clippy::needless_range_loop,
    clippy::default_trait_access,
    clippy::option_if_let_else,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]

pub mod day_solver;
pub mod days;
pub(crate) mod util;
