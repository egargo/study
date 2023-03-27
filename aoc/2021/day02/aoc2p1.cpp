/*
 * Advent of Code 2021
 * Clint <https://github.com/cliegargo>
 */


#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

int solution(std::vector<std::string> input) {
	int horizontal_pos = 0;
	int depth = 0;

	for(int i = 0; i < input.size(); ++i) {
		std::string dir = input[i].substr(0, input[i].size() - 2);
		int val = input[i].back() - '0';

		if(dir == "forward") {
			horizontal_pos += val;
		} else if(dir == "up") {
			depth -= val;
		} else if(dir == "down") {
			depth += val;
		}
	}

	return horizontal_pos * depth;
}


int main() {
	std::fstream lines;
	lines.open("sample.txt", std::ios::in);

	std::string s;

	std::vector<std::string> input;

	while(getline(lines, s)) {
		input.push_back(s);
	}

	std::cout << solution(input) << '\n';

	return 0;
}
