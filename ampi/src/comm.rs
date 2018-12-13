//! MPI communicator

use crate::{Rank, Result};

/// MPI communicator.
pub struct Comm(mpi_sys::MPI_Comm);

impl Comm {
    /// Get the rank of the current process in the communicator.
    pub fn rank<'s>(&'s self) -> Result<Rank<'s>> {
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
    pub fn last<'s>(&'s self) -> Result<Rank<'s>> {
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

    pub fn root<'s>(&'s self) -> Rank<'s> {
        Rank::new(0)
    }

    /// Is this rank the root rank of the communicator ?
    pub fn is_root(&self) -> Result<bool> {
        Ok(self.rank()? == self.root())
    }
}

impl Default for Comm {
    fn default() -> Self {
        Self(unsafe { mpi_sys::RSMPI_COMM_NULL })
    }
}
