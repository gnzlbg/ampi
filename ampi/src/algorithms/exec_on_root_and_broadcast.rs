//! Root do

use crate::{mem, Comm, Datatype, Result};

impl Comm {
    /// Executes `f` only in the root process of `comm`
    pub async fn exec_on_root_and_broadcast<
        's,
        R: Datatype + crate::fmt::Debug,
        F: Fn() -> R + 's,
    >(
        &'s self,
        f: F,
    ) -> Result<R>
    where
        mem::MaybeUninit<R>: Datatype,
    {
        let mut data = mem::MaybeUninit::uninitialized();
        if self.is_root()? {
            data.set(f());
        }
        println!("b = {:?}", unsafe { data.get_ref() });
        await!(self.broadcast(&mut data, self.root()))?;
        println!("a = {:?}", unsafe { data.get_ref() });
        Ok(unsafe { data.into_inner() })
    }
}
