pub struct Boxc<T> {
    p: *mut T,
}

impl<T> Boxc<T> {
    pub fn new(t: T) -> Self {
        return Boxc {
            p: Box::into_raw(Box::new(t)),
        };
    }
}

fn main() {
    let x = 42;
    let b = Boxc::new(x);
    println!("{:?}", b.p);
}
