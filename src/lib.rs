#[cfg(test)]
mod tests;

pub fn quick_sort<T: PartialOrd + Copy>(v: &mut Vec <T>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<T>::new();
    let mut right = Vec::<T>::new();

    let pivot = v.pop().unwrap();

    while let Some(x) = v.pop() {

        if x <= pivot {
            left.push(x);
        } else {
            right.push(x)
        }
    }

    quick_sort(&mut left);

    quick_sort(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}
// Seja v um mut Vec e left um mut Vec
// v.append(&mut left) esvazia left, deixando ele []

// Seja v um mut Vec e left um Vec
// v.extend(left.as_slice()) faz cópias dos elementos do slice
// v.extend_from_slice(left.as_slice()) faz clones dos elementos do slice

//--------------

pub fn stable_quick_sort<T: PartialOrd + Copy>(v: &mut Vec <T>) {

    if v.len() <= 1 {
        return;
    }

    let mut w = vec![];

    for (i, &x) in v.iter().enumerate() {
        w.push((x, i ))
    }

    quick_sort(&mut w);

    v.clear();

    for (x, i) in w {
        v.push(x);
    }
}

pub fn quick_sort_natural_stable<T: PartialOrd + Copy>(v: &mut Vec <T>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<T>::new();
    let mut right = Vec::<T>::new();

    let pivot = v.pop().unwrap();

    while let Some(x) = v.pop() {

        if x <= pivot {
            left.insert(0, x);  // O insert é muito mais lento que o push, pois realoca todas as posições
        } else {
            right.insert(0, x);
        }
    }

    quick_sort_natural_stable(&mut left);

    quick_sort_natural_stable(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}

pub fn quick_sort_natural_stable_fast<T: PartialOrd + Copy>(v: &mut Vec <T>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<T>::new();
    let mut right = Vec::<T>::new();

    let pivot = v.pop().unwrap();

    while let Some(x) = v.pop() {

        if x <= pivot {
            left.push(x);
        } else {
            right.push(x);
        }
    }

    left.reverse();

    right.reverse();

    quick_sort_natural_stable_fast(&mut left);

    quick_sort_natural_stable_fast(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}

pub fn quick_sort_natural_stable_proof_stability<T: PartialOrd + Copy>(v: &mut Vec <(T, usize)>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<(T, usize)>::new();
    let mut right = Vec::<(T, usize)>::new();

    let pivot = v.pop().unwrap();

    while let Some(x) = v.pop() {

        if x.0 <= pivot.0 {
            left.insert(0, x);
        } else {
            right.insert(0, x);
        }
    }

    quick_sort_natural_stable_proof_stability(&mut left);

    quick_sort_natural_stable_proof_stability(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}

pub fn quick_sort_natural_stable_fast_proof_stability<T: PartialOrd + Copy>(v: &mut Vec <(T, usize)>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<(T, usize)>::new();
    let mut right = Vec::<(T, usize)>::new();

    let pivot = v.pop().unwrap();

    while let Some(x) = v.pop() {

        if x.0 <= pivot.0 {
            left.push(x);
        } else {
            right.push(x);
        }
    }

    left.reverse();

    right.reverse();

    quick_sort_natural_stable_fast_proof_stability(&mut left);

    quick_sort_natural_stable_fast_proof_stability(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}


use rand;

pub fn randomized_quick_sort<T: PartialOrd + Copy>(v: &mut Vec <T>) {

    if v.len() <= 1 {
        return;
    }

    let mut left = Vec::<T>::new();
    let mut right = Vec::<T>::new();

    // As próximas 3 linhas são a parte randômica

    let k = v.len();

    let i = rand::random::<usize>() % k;

    let pivot = v.remove(i);

    while let Some(x) = v.pop() {

        if x <= pivot {
            left.push(x);
        } else {
            right.push(x)
        }
    }

    crate::quick_sort(&mut left);

    crate::quick_sort(&mut right);

    v.append(&mut left);

    v.push(pivot);

    v.append(&mut right);

}