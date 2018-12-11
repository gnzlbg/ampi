//! MPI Rank

use crate::{
    fmt,
    ops::{Deref, DerefMut},
};

/// MPI Rank
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(libc::c_uint);

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Rank {
    pub const fn new(x: libc::c_uint) -> Self {
        Self(x)
    }
}

impl From<libc::c_uint> for Rank {
    fn from(x: libc::c_uint) -> Self {
        Self(x)
    }
}

impl From<Rank> for libc::c_uint {
    fn from(x: Rank) -> Self {
        x.0
    }
}

impl From<libc::c_int> for Rank {
    fn from(x: libc::c_int) -> Self {
        Self(x as _)
    }
}

impl Deref for Rank {
    type Target = libc::c_uint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rank {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
