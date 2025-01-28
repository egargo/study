https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/c

#include <stdio.h>
#include <string.h>
#include <stddef.h>

size_t get_count(const char *s) {
	size_t count = 0;

	for(int i = 0; i < strlen(s); ++i) {
		if(s[i] == 97 || s[i] == 101 || s[i] == 105 || s[i] == 111 || s[i] == 117)
			count += 1;
	}

	return count;
}

int main() {
	char s[100];

	scanf("%[^\n]s", s);
	printf("%ld\n", get_count(s));

	return 0;
}
