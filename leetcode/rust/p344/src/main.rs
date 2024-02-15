struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut h = s.len() - 1;

        while l < h {
            let temp = s[l];
            s[l] = s[h];
            s[h] = temp;

            l += 1;
            h -= 1;
        }
    }
}

fn main() {
    Solution::reverse_string(&mut vec!['h', 'e', 'l', 'l', 'o']);
    Solution::reverse_string(&mut vec!['o', 'l', 'l', 'e', 'h']);
}
