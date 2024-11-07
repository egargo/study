// https://www.codewars.com/kata/517abf86da9663f1d2000003/train/java

public class ConvertStringToCamelCase {
	public static void main(String[] args) {
		String s = "the_Stealth_Warrior";
		s = "The_Stealth-Warrior";
		System.out.println(toCamelCase(s));

	}

	static String toCamelCase(String s) {
		String[] ssplit = s.replaceAll("[-]", "_").split("_");

		String camelCase = new String();
		for (int i = 0; i < ssplit.length; i++) {
			if (i == 0) {
				camelCase += ssplit[0];
			}

			if (i != 0) {
				camelCase += Character.toString(ssplit[i].charAt(0)).toUpperCase();
				camelCase += ssplit[i].substring(1, ssplit[i].length());
			}
		}

		return camelCase;
	}
}
