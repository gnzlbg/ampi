//! MP Future

use crate::{
    core_future,
    mem::{self, MaybeUninit},
    pin::{Pin, Unpin},
    request::Request,
    task::{LocalWaker, Poll},
};

/// MPI Future
pub struct Future<I, O> {
    pub input: I,
    pub output: MaybeUninit<O>,
    pub request: Request,
}

impl<I, O> Future<I, O> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            output: MaybeUninit::uninitialized(),
            request: Request::null(),
        }
    }
}

impl<I, O> Unpin for Future<I, O> {}

impl<I, O> core_future::Future for Future<I, O> {
    type Output = O;
    fn poll(mut self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        assert!(!self.request.is_null());

        match self.request.test() {
            Ok(r) => {
                if r {
                    let mut v = MaybeUninit::uninitialized();
                    mem::swap(&mut v, &mut self.output);
                    Poll::Ready(unsafe { v.into_inner() })
                } else {
                    lw.wake();
                    Poll::Pending
                }
            }
            Err(e) => panic!("testing the request failed with \"{:?}\"", e),
        }
    }
}
