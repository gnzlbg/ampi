//! MPI Environment

use crate::{ptr, Result};

pub struct Mpi;

impl Mpi {
    /// Has the MPI runtime been initialized ?
    pub fn is_init() -> Result<bool> {
        let r: Result<libc::c_int> = call! {
            MPI_Initialized(flag.as_mut_ptr()) => flag: libc::c_int
        };
        r.map(|v| v == 1)
    }
    pub fn init() -> Result<Mpi> {
        let r: Result<()> = call! {
            MPI_Init(ptr::null_mut(), ptr::null_mut())
        };
        r.map(|_| {
            let mpi = Mpi;
            assert!(
                Self::is_init().unwrap(),
                "unexpected failure to initialize the MPI runtime"
            );
            mpi
        })
    }
}

impl Drop for Mpi {
    fn drop(&mut self) {
        let r: Result<()> = call! {
            MPI_Finalize()
        };
        r.unwrap()
    }
}
