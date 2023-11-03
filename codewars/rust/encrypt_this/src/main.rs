// https://www.codewars.com/kata/5848565e273af816fb000449/train/rust

fn encrypt_this(text: &str) -> String {
    let mut encrypted: Vec<String> = Vec::new();

    let mut second = String::new();
    let mut third = String::new();

    for i in text.split(" ") {
        let size = i.len() - 1;

        for (j, k) in i.chars().enumerate() {
            let first = (i.chars().nth(0).unwrap() as u32).to_string();

            if size > 0 {
                second = i.chars().nth(size).unwrap().to_string();
                third = i.chars().nth(1).unwrap().to_string();
            }

            if j == 0 {
                encrypted.push(first.to_owned());
            } else if j == 1 {
                encrypted.push(second.to_owned());
            } else if j == size {
                encrypted.push(third.to_owned());
            } else {
                encrypted.push(k.to_string());
            }
        }

        encrypted.push(String::from(" "));
    }

    encrypted
        .into_iter()
        .map(|i| i)
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string()
}

fn main() {
    assert_eq!(encrypt_this(&"A"), "65".to_string());
    assert_eq!(
        encrypt_this(&"A wise old owl lived in an oak"),
        "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string()
    );
    assert_eq!(
        encrypt_this(&"The more he saw the less he spoke"),
        "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string()
    );
    assert_eq!(
        encrypt_this(&"The less he spoke the more he heard"),
        "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string()
    );
    assert_eq!(
        encrypt_this(&"Why can we not all be like that wise old bird"),
        "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string()
    );
    assert_eq!(
        encrypt_this(&"Thank you Piotr for all your help"),
        "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string()
    );
}
