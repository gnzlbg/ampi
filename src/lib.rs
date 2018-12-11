//! Non-blocking MPI wrapper
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
use core::{ffi, fmt, mem, pin, ptr, result, slice, str, task};

#[macro_use]
mod error;
pub use error::{Err, Result};

pub mod comm;
pub mod env;
pub mod future;
mod request;

pub mod algs;
