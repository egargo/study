// https://www.codewars.com/kata/55acfc59c3c23d230f00006d/train/c

#include <stdio.h>
#include <string.h>

int ascii_code(char ch) {
	return (int) ch;
}

int main() {
	char chars[] = { '\0', '\n', ' ', '0', 'A', 'a', '~', '\x7f' };

	for(int i = 0; i < sizeof(chars); i++) {
		printf("%d\n", ascii_code(chars[i]));
	}
}
