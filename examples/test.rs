#![feature(await_macro, async_await, pin, maybe_uninit)]

use ampi::exec;
use pin_utils::pin_mut;
use std::mem::MaybeUninit;

fn main() {
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

        Ok(rank_sum_x2)
    };
    let future = closure();
    pin_mut!(future);

    loop {
        if let Some(Ok(v)) = exec::poll_once(&mut future) {
            println!("Ready! => {}", v);
            break;
        } else {
            //println!("pending...");
        }
    }
}
