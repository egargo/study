// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/c

#include<stdio.h>

const char* even_or_odd(int number) {
	return (number % 2 == 0) ? "Even" : "Odd";
}

int main() {
	int number = 0;

	scanf("%d", &number);

	printf("%s\n", even_or_odd(number));
}
