// https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust

use std::collections::HashMap;

fn find_odd(arr: &[i32]) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for i in arr.iter() {
        if counter.get(i).is_some() {
            *counter.get_mut(i).unwrap() += 1;
        } else {
            counter.insert(*i, 1);
        }
    }

    for i in counter {
        if i.1 % 2 != 0 {
            return i.0;
        }
    }

    0
}

fn main() {
    assert_eq!(
        find_odd(&vec![
            20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
        ]),
        5
    );
    assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
    assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
    assert_eq!(find_odd(&vec![10]), 10);
    assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
    assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
}
