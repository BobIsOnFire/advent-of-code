#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_inline_in_public_items,
    clippy::single_call_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::fallible_impl_from,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

mod input;
pub use input::Solution;

pub mod util;
