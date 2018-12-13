//! Broadcast

use crate::{
    core_future, ffi::c_void, future::Future, Comm, Datatype, Rank, Result,
};

impl Comm {
    /// Broadcast `input_output` from `source_rank` to all other ranks in `self`.
    pub fn broadcast<'s, 'd: 's, I: Datatype + 'd>(
        &'s self,
        input_output: &'d mut I,
        source_rank: Rank<'s>,
    ) -> impl core_future::Future<Output = Result<()>> + 's {
        assert!(
            source_rank >= self.root() && source_rank < self.last().unwrap()
        );
        Future::new((input_output,), move |io, request| {
            let e = unsafe {
                mpi_sys::MPI_Ibcast(
                    io.0 as *mut _ as *mut c_void,
                    io.0.len() as _,
                    io.0.datatype(),
                    source_rank.into(),
                    *self.raw(),
                    request.raw(),
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
