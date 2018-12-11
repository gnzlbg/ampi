//! MPI communicator

use crate::Result;

/// MPI communicator.
#[derive(Copy, Clone)]
pub struct Comm(mpi_sys::MPI_Comm);

impl Comm {
    /// Returns the `MPI_COMM_WORLD` communicator.
    pub fn world() -> Self {
        debug_assert!(crate::env::Mpi::is_init().unwrap());
        Self(unsafe { mpi_sys::RSMPI_COMM_WORLD })
    }

    /// Get the rank of the current process in the communicator.
    pub fn rank(&self) -> Result<u32> {
        debug_assert!(crate::env::Mpi::is_init().unwrap());
        call! {
            MPI_Comm_rank(self.raw(), r.as_mut_ptr()) => r: libc::c_int
        }
    }

    pub fn raw(&self) -> mpi_sys::MPI_Comm {
        self.0
    }
}
