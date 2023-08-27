// https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust

fn find_missing_letter(chars: &[char]) -> char {
    let positions = chars.iter().map(|c| *c as u32).collect::<Vec<u32>>();

    let mut curr = positions[0] as u32;
    let mut missing = '0';

    for i in positions.iter() {
        if i == &curr {
            curr += 1;
        } else {
            missing = char::from_u32(curr).unwrap();
        }
    }

    missing
}

fn main() {
    assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
    assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
}
