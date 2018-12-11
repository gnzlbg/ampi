//! Error handling

use crate::{fmt, result, slice, str};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Err(libc::c_int);

/// MPI Result type
pub type Result<T> = result::Result<T, Err>;

impl Err {
    pub fn new(err: libc::c_int) -> Option<Self> {
        if err as libc::c_uint == mpi_sys::MPI_SUCCESS {
            None
        } else {
            Some(Self(err))
        }
    }
}

impl fmt::Debug for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = [b'\0'; 256];
        let mut len: libc::c_int = 0;

        let err = unsafe {
            mpi_sys::MPI_Error_string(
                self.0,
                buffer.as_mut_ptr() as *mut _,
                &mut len as *mut _,
            )
        };
        assert_eq!(
            err as libc::c_uint,
            mpi_sys::MPI_SUCCESS,
            "error while formatting error message"
        );
        assert!((len as usize) < buffer.len());

        let buffer =
            unsafe { slice::from_raw_parts(buffer.as_ptr(), len as _) };
        write!(f, "{}", str::from_utf8(buffer).unwrap())
    }
}

macro_rules! call {
    ($fn:ident($($args:expr),*) => $result:ident: $result_ty:ty) => {
        {
            #![allow(unused_mut)]
            use crate::mem;

            let mut $result = mem::MaybeUninit::<$result_ty>::uninitialized();
            if let Some(err) = crate::Err::new(unsafe { mpi_sys::$fn($($args),*) }) {
                Err(err)
            } else {
                Ok(unsafe { $result.into_inner() } as _)
            }
        }
    };
    ($fn:ident($($args:expr),*)) => {
        call!($fn($($args),*) => id: ())
    };
}
