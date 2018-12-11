//! MPI Request

use crate::Result;

#[derive(PartialEq, Eq, Debug)]
pub struct Request(mpi_sys::MPI_Request);

impl Request {
    /// Create new null MPI request.
    pub fn null() -> Self {
        Self(unsafe { mpi_sys::RSMPI_REQUEST_NULL })
    }

    pub fn is_null(&self) -> bool {
        *self == Self::null()
    }

    /// Tests whether the request is finished
    pub fn test(&mut self) -> Result<bool> {
        let ignore = unsafe { mpi_sys::RSMPI_STATUS_IGNORE };
        let r: Result<libc::c_int> = call! {
            MPI_Test(
                &mut self.0 as *mut mpi_sys::MPI_Request,
                flag.as_mut_ptr(),
                ignore
            ) => flag: libc::c_int
        };
        r.map(|v| v == 1)
    }

    pub fn ptr_mut(&mut self) -> *mut mpi_sys::MPI_Request {
        &mut self.0 as *mut _
    }
}
