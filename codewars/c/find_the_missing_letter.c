// https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/c

#include <stdio.h>
#include <string.h>

char findMissingLetter(char array[], int arrayLength) {
	char ascii[arrayLength];

	for(int i = 0; i < arrayLength; i++) {
		ascii[i] = (int) array[i];
	}

	int curr = ascii[0];

	for(int i = 0; i < arrayLength; ++i) {
		if( ascii[i] == curr ) {
			curr += 1;
		} else {
			return (char) curr;
		}
	}

	return ' ';
}


int main(int argc, char* argv[]) {
	char arr1[] = { 'a', 'b', 'c', 'd', 'f' };

	printf("%c\n", findMissingLetter(arr1, sizeof(arr1)));
}
