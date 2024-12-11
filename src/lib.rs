#![no_std]
extern crate alloc;

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

pub(crate) struct Once {
    inited: AtomicBool,
}

impl Once {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            inited: AtomicBool::new(false),
        }
    }

    #[inline]
    pub(crate) fn is_completed(&self) -> bool {
        self.inited.load(Ordering::Acquire)
    }

    #[cold]
    pub(crate) fn call_once(&self, f: impl FnOnce()) {
        if self.is_completed() {
            return;
        }

        critical_section::with(|_| {
            if !self.is_completed() {
                f();
                self.inited.store(true, Ordering::Relaxed);
            }
        });
    }
}

pub trait MutexInterface
where
    Self: Sized,
{
    fn create() -> Self;
    fn acquire(&self, max_wait: u32);
    fn release(&self);
}

pub struct Lock(u32);

impl MutexInterface for Lock {
    fn create() -> Self {
        Self(0xdeadbeef)
    }
    fn acquire(&self, _max_wait: u32) {}
    fn release(&self) {}
}

struct MutexInner<T: MutexInterface> {
    once: Once,
    inner: UnsafeCell<MaybeUninit<T>>,
}

impl<T: MutexInterface> MutexInner<T> {
    #[inline]
    const fn new() -> MutexInner<T> {
        MutexInner {
            once: Once::new(),
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    fn init(&self) {
        self.once.call_once(|| {
            let inner = T::create();
            unsafe { (*self.inner.get()).write(inner) };
        });
    }
}

struct MutexImpl<T: ?Sized, M: MutexInterface> {
    mutex: MutexInner<M>,
    data: UnsafeCell<T>,
}

impl<T, M: MutexInterface> MutexImpl<T, M> {
    /// Create a new mutex with the given inner value
    pub const fn new_const(t: T) -> Self {
        MutexImpl {
            mutex: MutexInner::new(),
            data: UnsafeCell::new(t),
        }
    }

    pub fn new(t: T) -> Self {
        let s = Self::new_const(t);
        s.mutex.init();
        s
    }
}

type Mutex<T> = MutexImpl<T, Lock>;

//use alloc::boxed::Box;
//
//pub fn static_bool() -> &'static mut Mutex<bool> {
//    Box::leak(Box::new(Mutex::new(false)))
//}

pub fn mutex_bool() -> Mutex<bool> {
    Mutex::new(false)
}

pub fn normal() {
    let _ = Mutex::new(false);
}

//fn main() {
//    let a = rx_buffer();
//    println!("{:?}", a.mutex.once.inited);
//}
