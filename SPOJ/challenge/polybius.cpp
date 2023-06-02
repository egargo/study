#include <bits/stdc++.h>
using namespace std;

vector<string> solution(vector<string> v) {
	map<char, int> alpha = {
		{'A', 11}, {'B', 12}, {'C', 13}, {'D', 14}, {'E', 15},
		{'F', 21}, {'G', 22}, {'H', 23}, {'I', 24}, {'J', 24}, {'K', 25},
		{'L', 31}, {'M', 32}, {'N', 33}, {'O', 34}, {'P', 35},
		{'Q', 41}, {'R', 42}, {'S', 43}, {'T', 44}, {'U', 45},
		{'V', 51}, {'W', 52}, {'X', 53}, {'Y', 54}, {'Z', 55},
	};

	vector<string> e;
	string tmp;

	for(int i = 0; i < v.size(); ++i) {
		for(int j = 0; j < v[i].size(); ++j) {
			auto it = alpha.find(v[i][j]);
			if(it != alpha.end()) {
				tmp += to_string(alpha.at(v[i][j])) + " ";
			} else {
				tmp += " ";
			}
		}
		e.push_back(tmp);
		tmp = "";
	}

	return e;
}

int main() {
	int t;
	string n = "";
	vector<string> v;

	cin>>t;

	for(int i = 0; i < t + 1; i++) {
		getline(cin, n);
		v.push_back(n);
	}

	vector<string> e = solution(v);

	for(int i = 0; i < e.size(); ++i) {
		cout << e[i] << '\n';
	}
}
