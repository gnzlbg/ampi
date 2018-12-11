//! MP Future

use crate::{
    core_future,
    mem::{self, MaybeUninit},
    pin::{Pin, Unpin},
    request::Request,
    task::{LocalWaker, Poll},
    Result,
};

pub struct State<I, O> {
    pub input: I,
    pub output: MaybeUninit<O>,
    pub request: Request,
}

/// MPI Future
pub struct Future<I, O, F>
where
    F: Fn(&mut State<I, O>) -> Result<()>,
{
    pub state: State<I, O>,
    pub function: F,
}

impl<I, O, F> Future<I, O, F>
where
    F: Fn(&mut State<I, O>) -> Result<()>,
{
    pub fn new(input: I, function: F) -> Self {
        Self {
            state: State {
                input,
                output: MaybeUninit::uninitialized(),
                request: Request::null(),
            },
            function,
        }
    }
}

impl<I, O, F> Unpin for Future<I, O, F> where
    F: Fn(&mut State<I, O>) -> Result<()>
{
}

impl<I, O, F> core_future::Future for Future<I, O, F>
where
    F: Fn(&mut State<I, O>) -> Result<()>,
{
    type Output = Result<O>;
    fn poll(mut self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        if self.state.request.is_null() {
            // really:
            match &mut *self {
                Self {
                    ref mut state,
                    ref function,
                } => {
                    if let Err(e) = function(state) {
                        return Poll::Ready(Err(e));
                    }
                }
            }
            assert!(!self.state.request.is_null());
        }

        match self.state.request.test() {
            Ok(r) => {
                if r {
                    let mut v = MaybeUninit::uninitialized();
                    mem::swap(&mut v, &mut self.state.output);
                    assert!(self.state.request.is_null());
                    Poll::Ready(Ok(unsafe { v.into_inner() }))
                } else {
                    lw.wake();
                    Poll::Pending
                }
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
