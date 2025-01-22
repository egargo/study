// https://www.codewars.com/kata/52223df9e8f98c7aa7000062/train/cpp

#include <iostream>
#include <string>

int wrap(const int max, int ascii) {
	while(ascii > max)
		ascii -= 26;
	return ascii;
}

std::string rot13(const std::string& str) {
	std::string ciphered;
	bool upper = false;
	int ascii = 0;

	for(auto letter : str) {
		if(std::isalpha(letter)) {
			ascii = ((int) letter) + 13;
			if(std::isupper(letter)) {
				if(ascii > 90)
					ascii = wrap(90, ascii);
			} else {
				if(ascii > 122)
					ascii = wrap(122, ascii);
			}
			ciphered += (char) ascii;
		} else {
			ciphered += letter;
		}
	}

	return ciphered;
}

int main() {
	std::string;

	std::cin >> str;
	std::cout << rot13(str) << '\n';

	return 0;
}
