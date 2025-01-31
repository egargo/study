// https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1/train/c

#include<stdio.h>
#include <stdlib.h>

int getAge(const char *inputString) {
	return atoi(inputString);
}

int main() {
	const char *s = "5 years old";
	printf("%d\n", getAge(s));

	return 0;
}
