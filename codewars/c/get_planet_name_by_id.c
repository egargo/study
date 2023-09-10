// https://www.codewars.com/kata/515e188a311df01cba000003/train/c

#include <stdio.h>

const char *getPlanetName(int id) {
	char *planets[] = {
		"Mercury", "Venus", "Earth", "Mars",
		"Jupiter", "Saturn", "Uranus", "Neptune"
	};

	return planets[id - 1];
}

int main(int argc, char *argv[]) {
	int id;
	scanf("%d", &id);

	printf("%s\n", getPlanetName(id));
}
