// https://www.codewars.com/kata/5264d2b162488dc400000001/train/c

#include<stdio.h>
#include<stdlib.h>
#include<string.h>

void spin_words(const char *sentence, char *result) {
	memset(result, 0, strlen(result));

	char *s = malloc(sizeof(char*) * sizeof(strlen(sentence)) * sizeof(char*));
	strcpy(s, sentence);
	char *token;
	char *temp = malloc(sizeof(char*) * sizeof(strlen(sentence)) * sizeof(char*));
	char *res = malloc(sizeof(char*) * sizeof(strlen(sentence)) * sizeof(char*));

	while((token = strtok_r(s, " ", &s))) {
		int n = strlen(token);
		if(n >= 5) {
			for(size_t i = strlen(token); i > 0; i--) {
				sprintf(temp, "%c", token[i]);
				strcat(res, temp);
			}
			sprintf(temp, "%c", token[0]);
			strcat(res, temp);
		} else {
			strcat(res, token);
		}
		memset(temp, 0, strlen(temp));
		strcat(res, " ");
	}

	strncat(result, res, strlen(res) - 1);
	printf("%s\n", result);

	free(temp);
	free(res);
}

int main() {
	char *sentence = "pariatur labore lorem excepteur magna ex esse occaecat tempor anim irure ea amet qui eiusmod voluptate lorem do ea ut quis officia cillum velit";
	char *result = malloc(sizeof(char*));
	spin_words(sentence, result);
}
