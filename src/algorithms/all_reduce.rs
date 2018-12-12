//! All reduce

use crate::{core_future, ffi::c_void, future::Future, Comm, Result};

impl Comm {
    pub fn all_reduce<'s>(
        &'s self,
        arg: u32,
    ) -> impl core_future::Future<Output = Result<u32>> + 's {
        Future::new(arg, move |state| {
            let e = unsafe {
                mpi_sys::MPI_Iallreduce(
                    &state.input as *const _ as *const c_void,
                    state.output.as_mut_ptr() as *mut c_void,
                    1,
                    mpi_sys::RSMPI_INT32_T,
                    mpi_sys::RSMPI_SUM,
                    *self.raw(),
                    state.request.raw(),
                )
            };

            if let Some(err) = crate::Err::new(e) {
                Err(err)
            } else {
                Ok(())
            }
        })
    }
}
