use super::{quick_sort, stable_quick_sort, quick_sort_natural_stable, quick_sort_natural_stable_fast,
            quick_sort_natural_stable_proof_stability, quick_sort_natural_stable_fast_proof_stability};

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

#[test]
fn test_quick_sort_natural_stable() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    quick_sort_natural_stable(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}
#[test]
fn test_quick_sort_natural_stable_fast() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    quick_sort_natural_stable_fast(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])
}


#[test]
fn test_quick_sort_natural_stable_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    quick_sort_natural_stable_proof_stability(&mut v);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])
}

#[test]
fn test_quick_sort_natural_stable_fast_proof_stability() {

    let mut v = vec![(10,1),(5,1),(2,1),(9,2),(0,2),(3,2),(3,1),(0,3),(9,1),(1,1),(5,3),(0,1),(10,2),(5,2),(6,1)];
    quick_sort_natural_stable_fast_proof_stability(&mut v);
    assert_eq!(v, [(0,2),(0,3),(0,1),(1,1),(2,1),(3,2),(3,1),(5,1),(5,3),(5,2),(6,1),(9,2),(9,1),(10,1),(10,2)])
}