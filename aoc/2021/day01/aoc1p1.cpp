/*
 * Advent of Code 2021
 * Clint <https://github.com/cliegargo>
 */


#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

int solution(std::vector<int> input) {
	int total = 0;

	for(int i = 0; i < input.size(); ++i) {
		if(input[i + 1] > input[i]) {
			total += 1;
		}
	}

	return total;
}


int main() {
	std::fstream lines;
	lines.open("input.txt", std::ios::in);

	std::string s;
	int n;

	std::vector<int> input;

	while(getline(lines, s)) {
		std::istringstream(s) >> n;
		input.push_back(n);
	}

	std::cout << solution(input) << '\n';

	return 0;
}
