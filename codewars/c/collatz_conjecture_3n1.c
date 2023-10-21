// https://www.codewars.com/kata/577a6e90d48e51c55e000217/train/c

#include <stdio.h>

unsigned hotpo(unsigned n) {
	unsigned sum = 0;

	while(n != 1) {
		if(n % 2 == 0) {
			n = n / 2;
		} else {
			n = 3 * n + 1;
		}

		sum += 1;
	}

	return sum;
}

int main() {
	unsigned n = 0;
	scanf("%u", &n);
	printf("%u\n", hotpo(n));
}
