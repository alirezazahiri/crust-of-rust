use super::Sorter;

pub struct QuickSort {
    smart: bool,
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len();

    while left < right {
        if &rest[left] <= pivot {
            // left already on the correct side
            left += 1;
        } else {
            right -= 1;
            // move element to the right side
            rest.swap(left, right);
        }
    }

    // place the pivot at its final location
    slice.swap(0, left);

    let (left, right) = slice.split_at_mut(left);

    assert!(left.last() <= right.first());

    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [unsorted | pivot | unsorted ]
        if !self.smart {
            quicksort(slice);
        } else {
            todo!();
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1, 5];
    QuickSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
