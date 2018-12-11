//! Non-blocking MPI algorithms

use crate::{comm, ffi::c_void, future::Future, Result};

pub fn all_reduce(
    arg: u32,
    comm: comm::Comm,
) -> Result<Box<Future<u32, u32>>> {
    let mut future = Box::new(Future::new(arg));
    let e = unsafe {
        mpi_sys::MPI_Iallreduce(
            &future.input as *const _ as *const c_void,
            future.output.as_mut_ptr() as *mut c_void,
            1,
            mpi_sys::RSMPI_INT32_T,
            mpi_sys::RSMPI_SUM,
            comm.raw(),
            future.request.ptr_mut(),
        )
    };

    if let Some(err) = crate::Err::new(e) {
        return Err(err);
    }

    Ok(future)
}
