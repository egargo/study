// https://www.codewars.com/kata/515f51d438015969f7000013/train/rust

fn pyramid(n: usize) -> Vec<Vec<u32>> {
    let mut pyramids: Vec<Vec<u32>> = Vec::new();
    let mut ones: Vec<u32> = Vec::new();

    for i in (0..n).into_iter() {
        for _ in (i..i + 1).into_iter() {
            ones.push(1);
        }
        pyramids.push(ones.clone());
    }

    pyramids
}

fn main() {
    assert_eq!(pyramid(0), vec![] as Vec<Vec<u32>>);
    assert_eq!(pyramid(1), vec![vec![1]]);
    assert_eq!(pyramid(2), vec![vec![1], vec![1, 1]]);
    assert_eq!(pyramid(3), vec![vec![1], vec![1, 1], vec![1, 1, 1]]);
}
