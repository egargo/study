// https://www.codewars.com/kata/5dd462a573ee6d0014ce715b/train/c

#include <stdio.h>
#include <ctype.h>

int same_case (char a, char b) {
	if (isalpha(a) && isalpha(b)) {
		if ((isupper(a) && isupper(b)) || (islower(a) && islower(b))) return 1;
		else return 0;
	} else {
		return -1;
	}
}


int main(int argc, char *argv[]) {
	printf("%d\n", same_case('C', 'B'));
	printf("%d\n", same_case('b', 'a'));
	printf("%d\n", same_case('d', 'd'));
	printf("%d\n", same_case('A', 's'));
	printf("%d\n", same_case('c', 'B'));
	printf("%d\n", same_case('b', 'Z'));
	printf("%d\n", same_case('\t', 'Z'));
	printf("%d\n", same_case('H', ':'));
}
