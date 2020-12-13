#[macro_export]
macro_rules! myvec {
    ($($element: expr),*) => {{
        #[allow(unused_mut)]
        let mut vec = Vec::new();
        $(vec.push($element);)*
        vec
    }};
    ($($element:expr,)*) => {{
        $crate::myvec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vec = Vec::with_capacity(count);
        vec.extend(::std::iter::repeat($element).take(count));
        vec
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = myvec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = myvec![10];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 10);
}

#[test]
fn double() {
    let x: Vec<u32> = myvec![10,20];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 20);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = myvec![10; 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 10);
}