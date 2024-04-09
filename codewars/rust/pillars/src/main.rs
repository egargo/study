// https://www.codewars.com/kata/5bb0c58f484fcd170700063d/train/rust

use std::cmp::Ordering;

fn pillars(num_of_pillars: u32, distance: u32, width: u32) -> u32 {
    match num_of_pillars.cmp(&1) {
        Ordering::Greater => {
            let d = (num_of_pillars - 1) * distance * 100;
            let w = (num_of_pillars - 2) * width;
            d + w
        }
        _ => 0,
    }
}

fn main() {
    assert_eq!(pillars(1, 10, 10), 0);
    assert_eq!(pillars(2, 20, 25), 2000);
    assert_eq!(pillars(11, 15, 30), 15270);
}
