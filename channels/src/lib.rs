use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// #[derive(Clone)]
// This de-sugars into `impl<T: Clone> Clone for Sender<T> { //...// }`
// which we DON't want the shared value of the Arc to become clone itself
// and only the Arc itself should be cloned
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
// INFO: Flavors:
// - Synchronous channels: Channel where send() can block. Limited capacity.
//  - Mutex + Condvar + VecDeque (sender blocks if it is full)
//  - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
// - Asynchronous channels: Channel where send() cannot block. Unbounded.
//  - Mutex + Condvar + VecDeque (sender does not block)
//  - Mutex + Condvar + LinkedList
//  - Atomic Queue (Linked List), i.e. linked list of T
//  - Atomic block linked list, i.e linked list of atomic VecDeque<T>
// - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization. e.g. unbuffered
// channels in Go
// - Oneshot channels: Any capacity. In practice, only one call to send(). e.g. you send something
// to another thread in order to shut it down

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn receive(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        tx.send(43);
        tx.send(44);
        tx.send(45);
        tx.clone().send(51);
        assert_eq!(rx.receive(), Some(42));
        assert_eq!(rx.receive(), Some(43));
        assert_eq!(rx.receive(), Some(44));
        assert_eq!(rx.receive(), Some(45));
        assert_eq!(rx.receive(), Some(51));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        {
            let _tx = tx;
        }
        // drop(tx);
        assert_eq!(rx.receive(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        {
            let _rx = rx;
        }
        // drop(rx);
        tx.send(42);
    }
}
