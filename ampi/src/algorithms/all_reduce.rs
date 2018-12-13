//! All reduce

use crate::{
    core_future, ffi::c_void, future::Future, mem, Comm, Compatible, Datatype,
    Result,
};

impl Comm {
    pub fn all_reduce<'s, 'd: 's, 'e: 's, I: Datatype + 'd, O: Datatype + 'e>(
        &'s self,
        input: &'d I,
        output: &'e mut O,
    ) -> impl core_future::Future<Output = Result<()>> + 's
    where
        O: Compatible<I>,
    {
        assert_eq!(input.datatype(), output.datatype());
        assert_eq!(input.len(), output.len());
        Future::new((input, output), move |state, request| {
            let e = unsafe {
                mpi_sys::MPI_Iallreduce(
                    state.0 as *const _ as *const c_void,
                    state.1 as *mut _ as *mut c_void,
                    state.0.len() as _,
                    state.0.datatype(),
                    mpi_sys::RSMPI_SUM,
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
    pub async fn all_reduce_<'s, I: Datatype + 's>(
        &'s self,
        input: I,
    ) -> Result<I>
    where
        mem::MaybeUninit<I>: Datatype,
    {
        use std::await;
        let mut output: mem::MaybeUninit<I> =
            mem::MaybeUninit::uninitialized();
        await!(self.all_reduce(&input, &mut output))?;
        Ok(unsafe { output.into_inner() })
    }
}
