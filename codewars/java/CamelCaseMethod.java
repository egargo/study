// https://www.codewars.com/kata/587731fda577b3d1b0001196/train/java

public class CamelCaseMethod {
	public static String camelCase(String str) {
		if (str.isEmpty() || str.isBlank()) {
			return "";
		}

		String camelCase = "";

		for (String string : str.trim().split(" +")) {
			camelCase += Character.toString(string.charAt(0)).toUpperCase();
			camelCase += string.substring(1, string.length());
		}

		return camelCase;
	}

	public static void main(String args[]) {
		String camelCase = camelCase(
				"  fuopgptubx vixajn uflcizavg xjzww hjpds wpfbtwnmid btw c jzwwlwc  praiiolrth njrgmn praiio uflcizav  vpbiaetym fuop cf olrthv");
		System.out.println(camelCase);
	}
}
