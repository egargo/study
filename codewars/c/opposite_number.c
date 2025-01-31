// https://www.codewars.com/kata/56dec885c54a926dcd001095/train/c

#include<stdio.h>

float opposite(float num) {
	return -num;
}

int main() {
	float num = 170.093857;
	printf("%.6f\n", opposite(num));
}
