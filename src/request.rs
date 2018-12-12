//! MPI Request

use crate::Result;

#[derive(PartialEq, Eq, Debug)]
pub struct Request(mpi_sys::MPI_Request);

impl Request {
    /// Create new null MPI request.
    pub fn null() -> Self {
        Self(unsafe { mpi_sys::RSMPI_REQUEST_NULL })
    }

    /// Is the MPI request null ?
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

    pub fn raw(&mut self) -> &mut mpi_sys::MPI_Request {
        &mut self.0
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        if self.0 == unsafe { mpi_sys::RSMPI_REQUEST_NULL } {
            return;
        }
        let r: Result<()> = call! {
            MPI_Request_free(&mut self.0 as *mut _) => r: ()
        };
        r.expect("failed to free request");
    }
}
