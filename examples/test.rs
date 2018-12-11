#![feature(await_macro, async_await)]

extern crate ampi;

use ampi::{algs, comm, env, exec};

fn main() {
    let _mpi = env::Mpi::init().unwrap();
    let comm = comm::Comm::world();
    let rank = comm.rank().unwrap();

    let c = async move || {
        let rank_sum = await!(algs::all_reduce(rank, comm)).unwrap();
        let mut count = 0_i32;
        let rank_sum1 = loop {
            count += 1;
            let rank_sum1 = await!(algs::all_reduce(rank, comm)).unwrap();
            if count == 100 {
                break rank_sum1;
            }
        };

        let rank_sum_x2 = rank_sum + rank_sum1;

        println!(
            "rank: {} => rank_sum: {} => x2: {}",
            rank, rank_sum, rank_sum_x2
        );

        rank_sum_x2
    };
    let mut future = c();

    let mut e = exec::Exec::new(&mut future);

    loop {
        if let Some(v) = e.poll() {
            println!("Ready! => {}", v);
            break;
        } else {
            println!("pending...");
        }
    }
}
