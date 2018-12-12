//! MP Future

use crate::{
    core_future,
    pin::{Pin, Unpin},
    request::Request,
    task::{LocalWaker, Poll},
    Result,
};

/// MPI Future
pub struct Future<S, F>
where
    F: Fn(&mut S, &mut Request) -> Result<()>,
{
    pub state: S,
    pub request: Request,
    pub function: F,
}

impl<S, F> Future<S, F>
where
    F: Fn(&mut S, &mut Request) -> Result<()>,
{
    pub fn new(state: S, function: F) -> Self {
        Self {
            state,
            request: Request::default(),
            function,
        }
    }
}

impl<S, F> Unpin for Future<S, F> where
    F: Fn(&mut S, &mut Request) -> Result<()>
{
}

impl<S, F> core_future::Future for Future<S, F>
where
    F: Fn(&mut S, &mut Request) -> Result<()>,
{
    type Output = Result<()>;
    fn poll(mut self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        if self.request.is_null() {
            match &mut *self {
                Self {
                    ref mut state,
                    ref mut request,
                    ref function,
                } => {
                    if let Err(e) = function(state, request) {
                        return Poll::Ready(Err(e));
                    }
                }
            }
            assert!(!self.request.is_null());
        }
        match self.request.test() {
            Ok(r) => {
                if r {
                    assert!(self.request.is_null());
                    Poll::Ready(Ok(()))
                } else {
                    lw.wake();
                    Poll::Pending
                }
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
