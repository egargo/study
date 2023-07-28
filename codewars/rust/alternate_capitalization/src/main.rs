// https://www.codewars.com/kata/59cfc000aeb2844d16000075/train/rust

fn capitalize(s: &str) -> Vec<String> {
    vec![
        s.chars()
            .enumerate()
            .map(|(u, c)| {
                if u % 2 == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(""),
        s.chars()
            .enumerate()
            .map(|(u, c)| {
                if u % 2 != 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(""),
    ]
}

fn main() {
    assert_eq!(capitalize("abcdef"), ["AbCdEf", "aBcDeF"]);
    assert_eq!(capitalize("codewars"), ["CoDeWaRs", "cOdEwArS"]);
    assert_eq!(capitalize("abracadabra"), ["AbRaCaDaBrA", "aBrAcAdAbRa"]);
    assert_eq!(capitalize("codewarriors"), ["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
}
