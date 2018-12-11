#![feature(libc, await_macro, async_await)]

extern crate ampi;
extern crate futures_executor;

use ampi::{algs, comm, env};

fn main() {
    let _mpi = env::Mpi::init().unwrap();
    let comm = comm::Comm::world();
    let rank = comm.rank().unwrap();

    futures_executor::block_on(
        async {
            let rank_sum = await!(algs::all_reduce(rank, comm)).unwrap();
            let rank_sum1 = await!(algs::all_reduce(rank, comm)).unwrap();

            let rank_sum_x2 = rank_sum + rank_sum1;

            println!(
                "rank: {} => rank_sum: {} => x2: {}",
                rank, rank_sum, rank_sum_x2
            );
        },
    );
}
