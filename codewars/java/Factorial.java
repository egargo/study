// https://www.codewars.com/kata/54ff0d1f355cfd20e60001fc/train/java

import java.lang.IllegalArgumentException;

public class Factorial {
	public static void main(String[] args) {
		Factorial factorial = new Factorial();
		System.out.println(factorial.factorial(5));
	}

	public int factorial(int n) throws IllegalArgumentException {
		if (n < 0 || n > 12) {
			throw new IllegalArgumentException("n must not be les than 0 or more than 12");
		}

		int f = 1;

		for (int i = n; i > 0; i--) {
			f *= i;
		}

		return f;
	}
}
