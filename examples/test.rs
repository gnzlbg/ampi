#![feature(await_macro, async_await, pin)]

use ampi::exec;
use pin_utils::pin_mut;

fn main() {
    let mpi = ampi::Env::init().unwrap();
    let comm = mpi.world();
    let rank = comm.rank().unwrap();

    let closure = async move || {
        let rank_sum = await!(comm.all_reduce(rank.into())).unwrap();
        let rank_sum1 = await!(comm.all_reduce(rank.into())).unwrap();

        let rank_sum_x2 = rank_sum + rank_sum1;

        println!(
            "rank: {} => rank_sum: {} => x2: {}",
            rank, rank_sum, rank_sum_x2
        );

        rank_sum_x2
    };
    let future = closure();
    pin_mut!(future);

    loop {
        if let Some(v) = exec::poll_once(&mut future) {
            println!("Ready! => {}", v);
            break;
        } else {
            println!("pending...");
        }
    }
}
