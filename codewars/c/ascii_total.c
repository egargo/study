// https://www.codewars.com/kata/572b6b2772a38bc1e700007a/train/c

#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int uni_total(const char *s) {
	int total = 0;
	for(size_t i=0;i<strlen(s);++i){
		total += s[i];
	}
	return total;
}

int main() {
	int n;
	scanf("%d\n", &n);

	char *s = malloc(sizeof(char) * (n + 1));
	scanf("%[^\n]s\n", s);

	int total = uni_total(s);
	printf("%d\n", total);
}
