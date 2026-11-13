#![feature(dropck_eyepatch)]

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Boxc<T> {
    p: NonNull<T>,
    // _marker: PhantomData<fn() -> T>,
    _marker: PhantomData<T>,
}

impl<T> Boxc<T> {
    pub fn new(t: T) -> Self {
        Boxc {
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _marker: PhantomData,
        }
    }
}

unsafe impl<#[may_dangle] T> Drop for Boxc<T> {
    // Safety: p was constructed from a Box in the first place, and has not been freed otherwise
    // since self still exists (otherwise, drop could not be called)
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p.as_mut()) };
    }
}

impl<T> Deref for Boxc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> DerefMut for Boxc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since
        // Also since we have &mut self, no other mutable reference has been given out to p
        unsafe { &mut *self.p.as_mut() }
    }
}

struct Oopsy<T: Debug>(T);

impl<T: Debug> Drop for Oopsy<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 52;
    let b = Boxc::new(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boxc::new(&mut y);
    println!("{:?}", b);

    let mut z = 42;
    let b = Boxc::new(Oopsy(&mut z));
    println!("{:?}", z);

    let s = String::from("hello");
    let mut boxc1 = Boxc::new(&*s);
    let boxc2: Boxc<&'static str> = Boxc::new("hello 2");
    boxc1 = boxc2;
}
