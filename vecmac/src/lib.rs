// () => {
//     Vec::new()
// };
// ($($element:expr),+ $(,)?) => {{
//     let mut vs = Vec::new();
//     $(vs.push($element);)*
//     vs
// }};

// #[macro_export]
// macro_rules! count_args {
//     () => {
//         0
//     };
//     ($($element:expr),*) => {{
//         let mut count = 0;
//         $($element; count += 1;)*
//         count
//     }};
//     ($($element:expr,)*) => {{
//         $crate::count_args!($($element),*)
//     }};
// }
#[macro_export]
#[doc(hidden)]
macro_rules! count_args {
    () => {
        0
    };
    ($($element:expr),*) => {{
        $crate::count_args!(@COUNT; $($element),*)
    }};
    ($($element:expr,)*) => {{
        $crate::count_args!($($element),*)
    }};
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count_args!(@SUBST; $element)),*])
    };
    (@SUBST; $_element:expr) => { () };
}

#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        let count = count_args!($($element),*);
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(count);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        // let count = $count;
        // let mut vs = Vec::with_capacity(count);
        // vs.extend(std::iter::repeat($element).take(count));
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};
}

#[test]
fn test_count_args() {
    let x = count_args!(1, 2, 3, 4);
    assert_eq!(x, 4);
}

#[test]
fn test_empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn test_single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

// #[test]
fn test_multiple() {
    let x: Vec<u32> = avec![1, 2, 3, 4];
    // assert!(!x.is_empty());
    // assert_eq!(x.len(), 4);
    // assert_eq!(x[0], 1);
    // assert_eq!(x[1], 2);
    // assert_eq!(x[2], 3);
    // assert_eq!(x[3], 4);
}

#[test]
fn test_trailing_comma() {
    let x: Vec<u32> = vec![1];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 1);
}

#[test]
fn test_multiple_count() {
    let x: Vec<u32> = avec![1; 3];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 1);
    assert_eq!(x[2], 1);
}

#[test]
fn test_multiple_count_non_literal() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 3];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

//-------------

#[allow(dead_code)]
trait MaxValue {
    fn max_val() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_val() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(u64);
max_impl!(u128);
max_impl!(i32);
max_impl!(i64);
max_impl!(i128);
max_impl!(f32);
max_impl!(f64);

#[test]
fn test_max_value() {
    let m = u32::max_val();
    assert_eq!(m, u32::MAX);
    let m = u64::max_val();
    assert_eq!(m, u64::MAX);
    let m = u128::max_val();
    assert_eq!(m, u128::MAX);
    let m = i32::max_val();
    assert_eq!(m, i32::MAX);
    let m = i64::max_val();
    assert_eq!(m, i64::MAX);
    let m = i128::max_val();
    assert_eq!(m, i128::MAX);
}
