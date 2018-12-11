use crate::{
    core_future::Future,
    pin::Pin,
    ptr::NonNull,
    task::{LocalWaker, Poll, UnsafeWake, Waker},
};

struct W;

unsafe impl UnsafeWake for W {
    unsafe fn clone_raw(&self) -> Waker {
        Waker::new(NonNull::from(self as &dyn UnsafeWake))
    }
    unsafe fn wake(&self) {}
    unsafe fn drop_raw(&self) {}
}

/// Executor
pub struct Exec<'a, F: Future> {
    future: Pin<&'a mut F>,
}

impl<'a, F: Future> Exec<'a, F> {
    pub fn new(future: &'a mut F) -> Self {
        // As long as we have a mut reference to the future,
        // nobody else can hold a reference to it, therefore
        // this is safe:
        Self {
            future: unsafe { Pin::new_unchecked(future) },
        }
    }

    /// Polls the future once
    pub fn poll(&mut self) -> Option<<F as Future>::Output> {
        let waker = W;
        let local_waker = unsafe { LocalWaker::new(NonNull::from(&waker)) };

        match self.future.as_mut().poll(&local_waker) {
            Poll::Pending => None,
            Poll::Ready(v) => Some(v),
        }
    }
}
