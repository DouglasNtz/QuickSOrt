use super::{quick_sort, stable_quick_sort};

#[test]
fn test_quick_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    quick_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}
#[test]
fn test_stable_quick_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    stable_quick_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}