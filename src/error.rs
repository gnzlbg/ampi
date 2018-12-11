//! Error handling
#![cfg_attr(feature = "cargo-clippy",
            allow(clippy::cast_sign_loss,
                  clippy::cast_possible_wrap))]

use crate::{fmt, num, result, slice, str};

// need to map libc::c_int into a NonZeroUXX type

trait NonZeroCInt {
    type T;
}
impl NonZeroCInt for i32 {
    type T = num::NonZeroU32;
}
impl NonZeroCInt for u32 {
    type T = num::NonZeroU32;
}
impl NonZeroCInt for i64 {
    type T = num::NonZeroU64;
}
impl NonZeroCInt for u64 {
    type T = num::NonZeroU64;
}

type NonZero = <libc::c_int as NonZeroCInt>::T;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Err(NonZero);

/// MPI Result type
pub type Result<T> = result::Result<T, Err>;

impl Err {
    pub fn new(err: libc::c_int) -> Option<Self> {
        assert_eq!(mpi_sys::MPI_SUCCESS, 0);
        if err as libc::c_uint == mpi_sys::MPI_SUCCESS {
            None
        } else {
            Some(Self(unsafe { NonZero::new_unchecked(err as _) }))
        }
    }
}

impl fmt::Debug for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = [b'\0'; 256];
        let mut len: libc::c_int = 0;

        let err = unsafe {
            mpi_sys::MPI_Error_string(
                self.0.get() as _,
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
            #![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_sign_loss))]

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
