//! Non-blocking MPI algorithms

use crate::{comm, core_future, ffi::c_void, future::Future, Result};

pub fn all_reduce(
    arg: u32,
    comm: comm::Comm,
) -> impl core_future::Future<Output = Result<u32>> {
    Future::new(arg, move |state| {
        let e = unsafe {
            mpi_sys::MPI_Iallreduce(
                &state.input as *const _ as *const c_void,
                state.output.as_mut_ptr() as *mut c_void,
                1,
                mpi_sys::RSMPI_INT32_T,
                mpi_sys::RSMPI_SUM,
                comm.raw(),
                state.request.ptr_mut(),
            )
        };

        if let Some(err) = crate::Err::new(e) {
            Err(err)
        } else {
            Ok(())
        }
    })
}
