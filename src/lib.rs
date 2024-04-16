#[cfg(test)]
mod tests;

pub fn my_insertion_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    let mut w = Vec::with_capacity(v.len());

    w.insert(0, v.pop().unwrap());

    let mut last_pos: usize = 0;

    while let Some(number) = v.pop() {
        find_insert_position(0, last_pos, &mut w, number);
        last_pos += 1;
    }

    *v = w;
}

fn my_insertion_sort_proof_stability<T: PartialOrd + Copy>(v: &mut Vec<(T,usize)>) {

    let mut w = Vec::with_capacity(v.len());

    w.insert(0, v.pop().unwrap());

    let mut last_pos: usize = 0;

    while let Some(number) = v.pop() {
        find_insert_position_proof_stability(0, last_pos, &mut w, number);
        last_pos += 1;
    }

    *v = w;
}

fn find_insert_position<T: PartialOrd + Copy>(mut i: usize, mut j: usize, w: &mut Vec<T>, number: T) {

    if j - i > 1 {
        if number <= w[(i + j)/2] {
            j = (i + j)/2 - 1;
        } else {
            i = (i + j)/2 + 1;
        }
        find_insert_position(i, j, w, number)
    } else if j - i == 0 {
        if number <= w[i] {
            w.insert(i, number);
        } else {
            w.insert(i + 1, number);
        };
    } else {
        if number <= w[i] {
            w.insert(i, number);
        } else if number <= w[j] {
            w.insert(j, number);
        } else {
            w.insert(j + 1, number)
        };
    }
}

fn find_insert_position_proof_stability<T: PartialOrd + Copy>(mut i: usize, mut j: usize, w: &mut Vec<(T,usize)>, number: (T,usize)) {

    if j - i > 1 {
        if number.0 <= w[(i + j)/2].0 {
            j = (i + j)/2 - 1;
        } else {
            i = (i + j)/2 + 1;
        }
        find_insert_position_proof_stability(i, j, w, number)
    } else if j - i == 0 {
        if number.0 <= w[i].0 {
            w.insert(i, number);
        } else {
            w.insert(i + 1, number);
        };
    } else {
        if number.0 <= w[i].0 {
            w.insert(i, number);
        } else if number.0 <= w[j].0 {
            w.insert(j, number);
        } else {
            w.insert(j + 1, number)
        };
    }
}


pub fn swap_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    'ext: for mut i in 1..v.len() {
        'int: loop {
            if v[i] < v[i-1] {
                v.swap(i, i - 1);
            } else {
                continue 'ext;
            }
            if i > 1 {
                i = i - 1;
                continue 'int;
            } else {
                continue 'ext;
            }
        }
    }
}

fn swap_sort_proof_stability<T: PartialOrd + Copy>(v: &mut Vec<(T,usize)>) {

    'ext: for mut i in 1..v.len() {
        'int: loop {
            if v[i].0 < v[i-1].0 {
                v.swap(i, i - 1);
            } else {
                continue 'ext;
            }
            if i > 1 {
                i = i - 1;
                continue 'int;
            } else {
                continue 'ext;
            }
        }
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    let mut w = Vec::with_capacity(v.len());

    w.insert(0, v.pop().unwrap());

    'ext: while let Some(number) = v.pop() {
        let mut pos = 0;
        while number > w[pos] {
            pos += 1;
            if pos == w.len() {
                w.insert(pos, number);
                continue 'ext;
            }
        }
        w.insert(pos, number);
    }

    *v = w;
}

fn insertion_sort_proof_stability<T: PartialOrd + Copy>(v: &mut Vec<(T,usize)>) {

    let mut w = Vec::with_capacity(v.len());

    w.insert(0, v.pop().unwrap());

    'ext: while let Some(number) = v.pop() {
        let mut pos = 0;
        while number.0 > w[pos].0 {
            pos += 1;
            if pos == w.len() {
                w.insert(pos, number);
                continue 'ext;
            }
        }
        w.insert(pos, number);
    }

    *v = w;
}

pub fn my_swap_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    for k in 1..v.len() {
        find_swap_position(0, k - 1, k, v)
    }
}

fn my_swap_sort_proof_stability<T: PartialOrd + Copy>(v: &mut Vec<(T,usize)>) {

    for k in 1..v.len() {
        find_swap_position_proof_stability(0, k - 1, k, v)
    }
}


fn find_swap_position<T: PartialOrd + Copy>(mut i: usize, mut j: usize, k: usize, w: &mut Vec<T>) {

    if j - i > 1 {
        if w[k] < w[(i + j)/2] {
            j = (i + j)/2 - 1;
        } else {
            i = (i + j)/2 + 1;
        }
        find_swap_position(i, j, k, w)
    } else if j - i == 0 {
        if w[k] < w[i] {
            w.insert(i, w[k]);
            w.remove( k + 1)
        } else {
            w.insert(i + 1, w[k]);
            w.remove( k + 1)
        };
    } else {
        if w[k] < w[i] {
            w.insert(i, w[k]);
            w.remove( k + 1)
        } else if w[k] < w[j] {
            w.insert(j, w[k]);
            w.remove( k + 1)
        } else {
            w.insert(j + 1, w[k]);
            w.remove( k + 1)
        };
    }
}

fn find_swap_position_proof_stability<T: PartialOrd + Copy>(mut i: usize, mut j: usize, k: usize, w: &mut Vec<(T,usize)>) {

    if j - i > 1 {
        if w[k].0 < w[(i + j)/2].0 {
            j = (i + j)/2 - 1;
        } else {
            i = (i + j)/2 + 1;
        }
        find_swap_position_proof_stability(i, j, k, w)
    } else if j - i == 0 {
        if w[k].0 < w[i].0 {
            w.insert(i, w[k]);
            w.remove( k + 1)
        } else {
            w.insert(i + 1, w[k]);
            w.remove( k + 1)
        };
    } else {
        if w[k].0 < w[i].0 {
            w.insert(i, w[k]);
            w.remove( k + 1)
        } else if w[k].0 < w[j].0 {
            w.insert(j, w[k]);
            w.remove( k + 1)
        } else {
            w.insert(j + 1, w[k]);
            w.remove( k + 1)
        };
    }
}

