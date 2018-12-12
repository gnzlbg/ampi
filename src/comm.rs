//! MPI communicator

use crate::{Rank, Result};

/// MPI communicator.
pub struct Comm(mpi_sys::MPI_Comm);

impl Comm {
    /// Get the rank of the current process in the communicator.
    pub fn rank(&self) -> Result<Rank> {
        let r = call! {
            MPI_Comm_rank(*self.raw(), r.as_mut_ptr()) => r: libc::c_int
        };
        r.map(|r: libc::c_int| r.into())
    }

    /// Obtains the raw communicator
    pub fn raw(&self) -> &mpi_sys::MPI_Comm {
        &self.0
    }

    /// Constructs a communicator from a raw communicator
    pub unsafe fn from_raw(comm: mpi_sys::MPI_Comm) -> Self {
        Self(comm)
    }

    /// Last rank in the communicator
    pub fn last(&self) -> Result<Rank> {
        let r = call! {
            MPI_Comm_size(*self.raw(), r.as_mut_ptr()) => r: libc::c_int
        };
        r.map(|r: libc::c_int| r.into())
    }

    /// Duplicates the communicator.
    pub fn try_clone(&self) -> Result<Self> {
        let r = call! {
            MPI_Comm_dup(*self.raw(), r.as_mut_ptr()) => r: mpi_sys::MPI_Comm
        };
        r.map(|r| Self(r))
    }
}

pub const fn root() -> Rank {
    Rank::new(0)
}
