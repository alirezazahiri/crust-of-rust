use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    thread::spawn,
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

#[derive(Debug)]
struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // !HACK: this is dangerous and NOT safe
        // while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        // self.locked.store(LOCKED, Ordering::Relaxed);
        // let ret = f(unsafe { &mut *self.v.get() });
        // self.locked.store(UNLOCKED, Ordering::Relaxed);
        // ret

        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol
            // A given location in memory can either be shared or exclusive
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(l.with_lock(|v| *v), 100 * 1000);
}
