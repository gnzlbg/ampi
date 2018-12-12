//! Projection from T to a type that can hold an uninitialized T

use crate::mem;

pub trait Uninitialized {
    type T;
    fn uninitialized() -> Self::T;
    fn as_mut_ptr(x: &mut Self::T) -> *mut Self;
    unsafe fn into_inner(x: Self::T) -> Self;
}

macro_rules! impl_single {
    ($id:ident) => {
        impl Uninitialized for $id {
            type T = mem::MaybeUninit<$id>;

            fn uninitialized() -> Self::T {
                Self::T::uninitialized()
            }
            fn as_mut_ptr(x: &mut Self::T) -> *mut Self {
                x.as_mut_ptr()
            }
            unsafe fn into_inner(x: Self::T) -> Self {
                x.into_inner()
            }

        }
    };
    ($($id:ident),*) => { $(impl_single!($id);)* }
}

impl_single! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}
