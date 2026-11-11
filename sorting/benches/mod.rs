use sorting::*;
use std::{
    cell::Cell,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct SortEvaluator<T> {
    t: T,
    cmps: Arc<AtomicUsize>,
}

impl<T> PartialEq for SortEvaluator<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmps.fetch_add(1, Ordering::AcqRel);
        self.t == other.t
    }
}
impl<T> Eq for SortEvaluator<T> where T: Eq {}

impl<T> PartialOrd for SortEvaluator<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.fetch_add(1, Ordering::Relaxed);
        self.t.partial_cmp(&other.t)
    }
}
impl<T> Ord for SortEvaluator<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.cmp(&other.t)
    }
}

fn main() {
    let rand = rand::thread_rng();
    let counter = Cell::new(0);
    for n in [0, 1, 10, 100, 1000, 10000] {
        for _ in 0..10 {
            let values = Vec::with_capacity(n);
            for _ in 0..n {
                values.push(SortEvaluator {
                    t: rand.gen(),
                    cmps: Rc::clone(&counter),
                });
            }

            let took = bench(BubbleSort, &values, &counter);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            let took = bench(SelectionSort, &values, &counter);
            let took = bench(QuickSort, &values, &counter);
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter>(sorter: S, values: &[T], counter: &Cell<usize>) -> usize {
    let mut values = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    counter.get()
}
