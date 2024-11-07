// https://www.codewars.com/kata/526571aae218b8ee490006f4/train/java

import java.util.stream.Collectors;

public class BitCounting {
	public static void main(String[] args) {
		int out = countBits(1234);
		System.out.println(out);
	}

	public static int countBits(int n) {
		return (Integer.toBinaryString(n)).chars().filter(c -> c == '1').mapToObj(Character::toString)
				.collect(Collectors.joining())
				.length();
	}
}
