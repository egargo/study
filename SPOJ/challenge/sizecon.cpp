#include <bits/stdc++.h>
using namespace std;

int solution(vector<int> v) {
	int s = 0;
	for(int i = 0; i < v.size(); ++i) {
		if(v[i] > 0)
			s += v[i];
	}
	return s;
}

int main() {
	int t, n;
	vector<int> v;
	cin>>t;
	for(int i = 0; i < t; ++i) {
		cin>>n;
		v.push_back(n);
	}
	cout<<solution(v);
}
