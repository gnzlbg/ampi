//! Asynchronous MPI wrapper
#![feature(
    async_await,
    arbitrary_self_types,
    futures_api,
    maybe_uninit,
    pin,
    stmt_expr_attributes,
    uniform_paths
)]
#![no_std]

use core::future as core_future;
use core::{ffi, fmt, mem, num, ops, pin, ptr, result, slice, str, task};

#[macro_use]
mod error;
pub use error::{Err, Result};

mod comm;
pub use comm::{root, Comm};

mod env;
pub use env::Env;

pub mod future;
mod request;

pub mod algorithms;
pub mod exec;
mod rank;
pub use rank::Rank;
