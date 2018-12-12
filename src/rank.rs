//! MPI Rank

use crate::{
    fmt, marker,
    ops::{Deref, DerefMut},
};

/// MPI Rank
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank<'s>(libc::c_uint, marker::PhantomData<&'s ()>);

impl<'a> fmt::Display for Rank<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Rank<'a> {
    pub const fn new(x: libc::c_uint) -> Self {
        Self(x, marker::PhantomData)
    }
}

impl<'a> From<libc::c_uint> for Rank<'a> {
    fn from(x: libc::c_uint) -> Self {
        Self(x, marker::PhantomData)
    }
}

impl<'a> From<Rank<'a>> for libc::c_uint {
    fn from(x: Rank<'a>) -> Self {
        x.0
    }
}

impl<'a> From<libc::c_int> for Rank<'a> {
    fn from(x: libc::c_int) -> Self {
        Self(x as _, marker::PhantomData)
    }
}

impl<'a> Deref for Rank<'a> {
    type Target = libc::c_uint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Rank<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
