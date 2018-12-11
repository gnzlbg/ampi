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

/// Polls the future once
pub fn poll_once<F: Future>(
    future: &mut Pin<&mut F>,
) -> Option<<F as Future>::Output> {
    let waker = W;
    let local_waker = unsafe { LocalWaker::new(NonNull::from(&waker)) };

    match future.as_mut().poll(&local_waker) {
        Poll::Pending => None,
        Poll::Ready(v) => Some(v),
    }
}
