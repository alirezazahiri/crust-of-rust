use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if !self.smart {
                // let mut i = unsorted;
                // while i > 0 && slice[i - 1] > slice[i] {
                //     slice.swap(i - 1, i);
                //     i -= 1;
                // }
                for i in (1..=unsorted).rev() {
                    if slice[i - 1] > slice[i] {
                        slice.swap(i - 1, i);
                    }
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // [a, c, e].binary_search(c) => 1
                    Ok(i) => i,
                    // [a, c, e].binary_search(b) => Err(1) => it reaches an index where the item
                    // should be, BUT its not
                    Err(i) => i,
                };

                // shift the slice from the given insertion point,
                // 1 position to the right inside the slice from 0 to the unsorted item's position
                // which makes the unsorted item go out and since it is a rotation, it comes back at
                // the first position which is the i-th position that is the insertion point we
                // wanted
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

#[test]
fn it_works_dumb() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
