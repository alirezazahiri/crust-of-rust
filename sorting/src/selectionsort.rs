use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted ]
        for unsorted in 0..slice.len() {
            let smallest_in_rest_idx = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("array is non-empty");

            // let mut smallest_in_rest_idx = unsorted;
            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest_idx] {
            //         smallest_in_rest_idx = i;
            //     }
            // }

            if unsorted != smallest_in_rest_idx {
                slice.swap(unsorted, smallest_in_rest_idx);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
