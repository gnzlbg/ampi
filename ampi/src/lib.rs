//! Asynchronous MPI wrapper
#![feature(
    async_await,
    await_macro,
    arbitrary_self_types,
    futures_api,
    maybe_uninit,
    pin,
    stmt_expr_attributes,
    uniform_paths,
    custom_attribute
)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(ampi_test_runner::runner))]
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

#[cfg(test)]
mod tests {
    use ampi_test_macro::mpi_test;
    use pin_utils::pin_mut;

    #[mpi_test(4)]
    fn test_success() {
        use crate as ampi;
        use std::mem::MaybeUninit;
        use ampi::exec;

        let mpi = ampi::Env::init().unwrap();
        let comm = mpi.world();

        let closure = async move || -> ampi::Result<u32> {
            let rank = comm.rank()?;
            let rank_value: u32 = rank.into();
            let mut rank_sum: MaybeUninit<u32> = MaybeUninit::uninitialized();
            await!(comm.all_reduce(&rank_value, &mut rank_sum))?;
            let rank_sum = unsafe { rank_sum.into_inner() };
            let rank_sum1 = await!(comm.all_reduce_(rank_value))?;
            let rank_sum_x2 = rank_sum + rank_sum1;
            let rank_sum_x4 =
                await!(comm.exec_on_root_and_broadcast(|| rank_sum_x2 * 2))?;

            println!(
                "rank: {} => s: ({} == {}), x2: {}, x4: {}",
                rank, rank_sum, rank_sum1, rank_sum_x2, rank_sum_x4
            );

            Ok(rank_sum_x4)
        };
        let future = closure();
        pin_mut!(future);

        loop {
            if let Some(Ok(v)) = exec::poll_once(&mut future) {
                println!("Ready! => {}", v);
                assert_eq!(v, 24);
                break;
            } else {
                //println!("pending...");
            }
        }
    }
}
