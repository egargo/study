/*
 * Advent of Code 2021
 * Clint <https://github.com/cliegargo>
 */


#include <iostream>
#include <fstream>
#include <vector>
#include <tuple>
#include <sstream>

int solution(std::vector<int> input) {
	std::vector<int> sum;

	for(int i = 0; i < input.size() - 2; ++i) {
		int f_term = input[i];
		int s_term = input[i + 1];
		int t_term = input[i + 2];

		int t_sum = f_term + s_term + t_term;

		sum.push_back(t_sum);

		f_term = s_term;
		s_term = t_term;
		t_term = input[i + 1];
	}

	int total = 0;

	for(int i = 0; i < sum.size(); ++i) {
		if(sum[i + 1] > sum[i]) {
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
