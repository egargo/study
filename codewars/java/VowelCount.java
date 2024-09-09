// https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/java

import java.util.HashMap;

public class VowelCount {
	public static void main(String[] args) {
		System.out.println(getCount("abracadabra"));
	}

	public static int getCount(String str) {
		HashMap<Character, Integer> vowels = new HashMap<>();
		vowels.put('a', 1);
		vowels.put('e', 1);
		vowels.put('i', 1);
		vowels.put('o', 1);
		vowels.put('u', 1);

		int count = 0;
		for (int i = 0; i < str.length(); ++i) {
			char letter = str.charAt(i);
			if (vowels.get(letter) != null) {
				count += vowels.get(letter);
			}
		}
		return count;
	}
}
