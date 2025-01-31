// https://www.codewars.com/kata/52223df9e8f98c7aa7000062/train/c

#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include <ctype.h>

#define ROT 13
#define UPPER_MAX 90
#define LOWER_MAX 122

int wrap(int ascii, int max) {
	while(ascii > max) ascii -= 26;
	return ascii;
}

char *rot13(char *str_out, const char *str_in) {
	*str_out = '\0';
	char *temp = malloc(sizeof(char*));

	for(size_t i = 0; i < strlen(str_in); ++i) {
		int ascii = str_in[i];
		char letter = (char) str_in[i];

		if(isalpha(letter) != 0) {
			int ascii_rot = ascii + ROT;
			if(isupper(letter)) {
				if(ascii_rot > UPPER_MAX) ascii_rot = wrap(ascii_rot, UPPER_MAX);
				sprintf(temp, "%c", (char) ascii_rot);
				strcat(str_out, temp);
			} else {
				if(ascii_rot > LOWER_MAX) ascii_rot = wrap(ascii_rot, LOWER_MAX);
				sprintf(temp, "%c", (char) ascii_rot);
				strcat(str_out, temp);
			}
		} else {
			sprintf(temp, "%c", letter);
			strcat(str_out, temp);
		}
		*temp = '\0';
	}

	free(temp);
	return str_out;
}

int main() {
	char *str_in = "EBG13 rknzcyr.";
	char *str_out = malloc(sizeof(char*) * sizeof(strlen(str_in)) + 1);

	char *t = rot13(str_out, str_in);
	printf("%s\n", t);

	return 0;
}
