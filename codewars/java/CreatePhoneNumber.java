// https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/java

public class CreatePhoneNumber {
	public static void main(String[] args) {
		System.out.println(createPhoneNumber(new int[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 0 }));
	}

	public static String createPhoneNumber(int[] numbers) {
		String result = new String();
		;
		for (int i = 0; i < numbers.length; ++i) {
			if (i == 0)
				result += "(";

			if (i == 3)
				result += ") ";

			if (i == 6)
				result += "-";

			result += numbers[i];
		}

		return result;
	}
}
