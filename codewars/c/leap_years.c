// https://www.codewars.com/kata/526c7363236867513f0005ca/train/c

#include<stdio.h>
#include <stdbool.h>

bool IsLeapYear(int year) {
	if (year % 400 == 0 || year % 4 == 0 && year % 100 != 0) return true;
}

int main() {
	int year;
	scanf("%d", &year);
	printf("%d\n", IsLeapYear(year));
	return 0;
}
