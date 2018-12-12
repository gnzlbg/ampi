//! Asynchronous MPI wrapper
#![feature(
    async_await,
    await_macro,
    arbitrary_self_types,
    futures_api,
    maybe_uninit,
    pin,
    stmt_expr_attributes,
    uniform_paths
)]
//#![no_std]  // TODO: because of std::await

extern crate core;
use core::future as core_future;
use core::{
    ffi, fmt, marker, mem, num, ops, pin, ptr, result, slice, str, task,
};

#[macro_use]
mod error;
pub use error::{Err, Result};

mod comm;
pub use comm::Comm;

mod env;
pub use env::Env;

pub mod future;
mod request;

mod algorithms;
pub mod exec;
mod rank;
pub use rank::Rank;

mod datatype;
pub use datatype::Datatype;

mod uninitialized;
pub use uninitialized::Uninitialized;

mod compatible;
pub use compatible::Compatible;
