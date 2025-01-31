// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/c

#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<ctype.h>

char *accum(const char *source) {
	int size = -2;
	for(int i = 0; i < strlen(source) + 2; ++i) size += i;

	char *temp = malloc(sizeof(char*));
	char *mumble = malloc(sizeof(char*) * size);
	*mumble = '\0';

	int count = 1;

	for(size_t i = 0; i < strlen(source); ++i) {
		for(int j = 0; j < count; ++j) {
			if(j == 0) sprintf(temp, "%c", toupper((char) source[i]));
			else sprintf(temp, "%c", tolower( (char) source[i]));
			strcat(mumble, temp);
		}

		if(i + 1 != strlen(source)) strcat(mumble, "-");

		count += 1;
		*temp = '\0';
	}

	free(temp);

	return mumble;
}

int main() {
	// const char *source = "abcd";
	const char *source = "ZpglnRxqenU";
	char *mumble = accum(source);
	printf("%s\n", mumble);
	return 0;
}
