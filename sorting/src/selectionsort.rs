use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted ]
        for unsorted in 0..slice.len() {
            let min_idx = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(result, _)| result + unsorted)
                .expect("array is non-empty");

            slice.swap(min_idx, unsorted);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
