//! MPI_Datatypes

use crate::mem;

/// MPI Datatype
pub unsafe trait Datatype {
    fn datatype(&self) -> mpi_sys::MPI_Datatype;
    fn as_mut_ptr(&mut self) -> *mut Self {
        self
    }
    fn as_ptr(&self) -> *const Self {
        self
    }
    fn len(&self) -> usize {
        1
    }
}

macro_rules! impl_primitive {
    ($id:ident => $mpi:ident) => {
        unsafe impl Datatype for $id {
            fn datatype(&self) -> mpi_sys::MPI_Datatype {
                unsafe { mpi_sys::$mpi }
            }
        }

        unsafe impl Datatype for mem::MaybeUninit<$id> {
            fn datatype(&self) -> mpi_sys::MPI_Datatype {
                unsafe { mpi_sys::$mpi }
            }
        }
    };
}

impl_primitive!(u32 => RSMPI_UINT32_T);
impl_primitive!(i32 => RSMPI_INT32_T);
