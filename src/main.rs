fn main() {
    let mut numbers: Vec<i32> = vec![-5, 10, 2, -1, 0, 99, 100, 83, 782, 1, 33, 50];
    let right = &numbers.len() - 1 as usize;

    
    // let sorted_numbers = insertion_sort(&mut numbers);
    // let sorted_numbers = bubble_sort(&mut numbers);
    // let sorted_numbers = selection_sort(&mut numbers);
    // let sorted_numbers = merge_sort(&mut numbers);
    let sorted_numbers = quick_sort(&mut numbers, 0, right);
    
    // println!("Before: {:?}", numbers);
    println!("After:  {:?}\n", sorted_numbers);
}

// Time complexity: O(n^2)
fn bubble_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }

    vec.to_vec()
}

// Time complexity: O(n^2)
fn insertion_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }

    vec.to_vec()
}

// Time complexity: O(n^2)
fn selection_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        let mut min = i;

        for j in i + 1..vec.len() {
            if vec[j] < vec[min] {
                min = j;
            }
        }

        if min != i {
            vec.swap(i, min);
        }
    }

    vec.to_vec()
}

// Time complexity: O(n log(n))
fn merge_sort(vec: &mut Vec<i32>) -> Vec<i32> {
    let mid = vec.len() / 2;

    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let left = merge_sort(&mut vec[0..mid].to_vec());
        let right = merge_sort(&mut vec[mid..].to_vec());

        merge(left, right)
    }
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}

// Time complexity: O(n log(n))
fn quick_sort(vec: &mut Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if left >= right {
        return vec.to_vec();
    }

    let pivot_index = partition(vec, left, right);
    
    quick_sort(vec, left, pivot_index - 1);
    quick_sort(vec, pivot_index + 1, right);

    vec.to_vec()
}

fn partition(vec: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_value = vec[right];
    let mut partition_index = left;

    for i in left..right {
        if vec[i] < pivot_value {
            vec.swap(i, partition_index);
            partition_index += 1;
        }
    }

    vec.swap(right, partition_index);

    partition_index
}