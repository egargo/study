// https://www.codewars.com/kata/54da5a58ea159efa38000836/train/java

import java.util.HashMap;
import java.util.Map;

public class FindTheOddInt {
	public static void main(String[] args) {
		int[] n = new int[] { 20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5 };
		int out = findIt(n);
		System.out.println(out);
	}

	public static int findIt(int[] a) {
		HashMap<Integer, Integer> map = new HashMap<>();

		for (int i : a) {
			if (map.get(i) == null)
				map.put(i, 1);
			else
				map.put(i, map.get(i) + 1);
		}

		int odd = 0;

		for (Map.Entry<Integer, Integer> entry : map.entrySet()) {
			if (entry.getValue() % 2 != 0) {
				odd = entry.getKey();
				break;
			}
		}

		return odd;
	}
}
