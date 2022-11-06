// https://www.codewars.com/kata/54b724efac3d5402db00065e/train/rust


use std::collections::HashMap;
use regex::Regex;

fn decode_morse(encoded: &str) -> String {
	let MORSE_CODE = HashMap::from([
		(".-".to_string(), "A".to_string()),
		("-...".to_string(), "B".to_string()),
		("-.-.".to_string(), "C".to_string()),
		("-..".to_string(), "D".to_string()),
		(".".to_string(), "E".to_string()),
		("..-.".to_string(), "F".to_string()),
		("--.".to_string(), "G".to_string()),
		("....".to_string(), "H".to_string()),
		("..".to_string(), "I".to_string()),
		(".---".to_string(), "J".to_string()),
		("-.-".to_string(), "K".to_string()),
		(".-..".to_string(), "L".to_string()),
		("--".to_string(), "M".to_string()),
		("-.".to_string(), "N".to_string()),
		("---".to_string(), "O".to_string()),
		(".--.".to_string(), "P".to_string()),
		("--.-".to_string(), "Q".to_string()),
		(".-.".to_string(), "R".to_string()),
		("...".to_string(), "S".to_string()),
		("-".to_string(), "T".to_string()),
		("..-".to_string(), "U".to_string()),
		("...-".to_string(), "V".to_string()),
		(".--".to_string(), "W".to_string()),
		("-..-".to_string(), "X".to_string()),
		("-.--".to_string(), "Y".to_string()),
		("--..".to_string(), "Z".to_string()),
		("-----".to_string(), "0".to_string()),
		(".----".to_string(), "1".to_string()),
		("..---".to_string(), "2".to_string()),
		("...--".to_string(), "3".to_string()),
		("....-".to_string(), "4".to_string()),
		(".....".to_string(), "5".to_string()),
		("-....".to_string(), "6".to_string()),
		("--...".to_string(), "7".to_string()),
		("---..".to_string(), "8".to_string()),
		("----.".to_string(), "9".to_string()),
		(".-.-.-".to_string(), ".".to_string()),
		("--..--".to_string(), ".to_string(),".to_string()),
		("..--..".to_string(), "?".to_string()),
		(".----.".to_string(), "'".to_string()),
		("-.-.--".to_string(), "!".to_string()),
		("-..-.".to_string(), "/".to_string()),
		("-.--.".to_string(), "(".to_string()),
		("-.--.-".to_string(), ")".to_string()),
		(".-...".to_string(), "&".to_string()),
		("---...".to_string(), ".to_string(),".to_string()),
		("-.-.-.".to_string(), ";".to_string()),
		("-...-".to_string(), "=".to_string()),
		(".-.-.".to_string(), "+".to_string()),
		("-....-".to_string(), "-".to_string()),
		("..--.-".to_string(), "_".to_string()),
		(".-..-.".to_string(), "\"".to_string()),
		("...-..-".to_string(), "$".to_string()),
		(".--.-.".to_string(), "@".to_string()),
		("...---...".to_string(), "SOS".to_string()),
	]);

	if encoded.trim().is_empty() {
		return "".to_string()
	}

	let mut decoded = String::new();

	encoded.trim().split(" ").collect::<Vec<_>>().iter().for_each(|i| {
		if MORSE_CODE.get(*i) != None {
			decoded.push_str(MORSE_CODE.get(*i).unwrap());
		} else {
			decoded.push_str(" ");
		}
	});

	Regex::new(r"[\s]+").unwrap().replace_all(&decoded, " ").to_string()
}


fn main() {
	let encoded = String::from(".... . -.--   .--- ..- -.. .");
    println!("{}", decode_morse(&encoded));
}
