// https://www.codewars.com/kata/563a631f7cbbc236cf0000c2/train/rust

fn move_hero(position: u32, roll: u32) -> u32 {
    position + (roll * 2)
}

fn main() {
    assert_eq!(move_hero(0, 4), 8);
}
