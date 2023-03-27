/*
 * Advent of Code 2021
 * Clint <https://github.com/cliegargo>
 */


#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

int get_gamma(std::vector<std::string> input, std::vector<std::pair<int, int>> common) {
	std::string temp;
	int gamma = 0;
	std::vector<int> bit_count = {2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1};
	for(int i = 0; i < 12; ++i) {
		int x = common[i].first;
		int y = common[i].second;

		for(int j = 0; j < input.size(); ++j) {
			if(x > y) {
				if(input[j][i] == '1') {
					gamma += bit_count[i];
					break;
				}
			}
		}
	}

	return gamma;
}

int get_epsilon(std::vector<std::string> input, std::vector<std::pair<int, int>> common) {
	std::string temp;
	int epsilon = 0;
	std::vector<int> bit_count = {2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1};

	for(int i = 0; i < 12; ++i) {
		int x = common[i].first;
		int y = common[i].second;

		for(int j = 0; j < input.size(); ++j) {
			if(x < y) {
				if(input[j][i] == '1') {
					epsilon += bit_count[i];
					break;
				}
			}
		}
	}

	return epsilon;
}

int solution(std::vector<std::string> input) {
	int ones = 0;
	int zeros = 0;

	std::vector<std::pair<int, int>> common;

	for(int i = 0; i < 12; ++i) {
		for(int j = 0; j < input.size(); ++j) {
			if(input[j][i] == '1') {
				ones += 1;
			} else {
				zeros += 1;
			}
		}
		common.push_back(std::make_pair(ones, zeros));
		ones = 0;
		zeros = 0;
	}

	int gamma = get_gamma(input, common);
	int epsilon = get_epsilon(input, common);

	return gamma * epsilon;
}


int main() {
	std::fstream lines;
	lines.open("input.txt", std::ios::in);

	std::string s;

	std::vector<std::string> input;

	while(getline(lines, s)) {
		input.push_back(s);
	}

	std::cout << solution(input) << '\n';

	return 0;
}
