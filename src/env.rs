//! MPI Environment

use crate::{ptr, Comm, Result};

pub struct Env(());

impl Env {
    /// Has the MPI runtime been initialized ?
    pub fn is_init() -> Result<bool> {
        let r: Result<libc::c_int> = call! {
            MPI_Initialized(flag.as_mut_ptr()) => flag: libc::c_int
        };
        r.map(|v| v == 1)
    }
    pub fn init() -> Result<Self> {
        let r: Result<()> = call! {
            MPI_Init(ptr::null_mut(), ptr::null_mut())
        };
        r.map(|_| {
            let mpi = Self(());
            assert!(
                Self::is_init().unwrap(),
                "unexpected failure to initialize the MPI runtime"
            );
            mpi
        })
    }

    /// Returns the `MPI_COMM_WORLD` communicator.
    pub fn world(&self) -> Comm {
        unsafe { Comm::from_raw(mpi_sys::RSMPI_COMM_WORLD) }
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let r: Result<()> = call! {
            MPI_Finalize()
        };
        r.unwrap()
    }
}
