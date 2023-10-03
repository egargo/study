// https://www.codewars.com/kata/6512b3775bf8500baea77663/train/rust

fn gimme_the_letters(sp: &str) -> String {
    let first = sp.chars().nth(0).unwrap() as u8;
    let last = sp.chars().nth(sp.len() - 1).unwrap() as u8;
    let mut letters = String::new();

    for i in first..=last {
        letters.push(i as char);
    }

    letters
}

fn main() {
    assert_eq!(gimme_the_letters("a-z"), "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(gimme_the_letters("h-o"), "hijklmno");
    assert_eq!(gimme_the_letters("Q-Z"), "QRSTUVWXYZ");
    assert_eq!(gimme_the_letters("J-J"), "J");
    assert_eq!(gimme_the_letters("a-b"), "ab");
    assert_eq!(gimme_the_letters("A-A"), "A");
    assert_eq!(gimme_the_letters("g-i"), "ghi");
    assert_eq!(gimme_the_letters("H-I"), "HI");
    assert_eq!(gimme_the_letters("y-z"), "yz");
    assert_eq!(gimme_the_letters("e-k"), "efghijk");
    assert_eq!(gimme_the_letters("a-q"), "abcdefghijklmnopq");
    assert_eq!(gimme_the_letters("F-O"), "FGHIJKLMNO");
}
