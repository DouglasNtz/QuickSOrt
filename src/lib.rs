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