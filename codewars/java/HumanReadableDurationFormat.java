import java.util.ArrayList;

public class HumanReadableDurationFormat {
	public static String toSecs(int seconds) {
		int ss = seconds % 60;

		if (ss != 0)
			return (ss > 1) ? ss + " seconds" : ss + " second";

		return new String();
	}

	public static String toMins(int seconds) {
		int mm = (seconds % 3600) / 60;

		if (mm != 0)
			return (mm > 1) ? mm + " minutes" : mm + " minute";

		return new String();
	}

	public static String toHours(int seconds) {
		int hh = (seconds <= 3600) ? seconds / 3600 : (seconds % 86400) / 3600;

		if (hh != 0)
			return (hh > 1) ? hh + " hours" : hh + " hour";

		return new String();
	}

	public static String toDays(int seconds) {
		int dd = (seconds % 31536000) / 86400;

		if (dd != 0)
			return (dd > 1) ? dd + " days" : dd + " day";

		return new String();
	}

	public static String toYears(int seconds) {
		int yy = seconds / 31536000;

		if (yy != 0)
			return (yy > 1) ? yy + " years" : yy + " year";

		return new String();
	}

	public static String formatDuration(int seconds) {
		if (seconds == 0)
			return "now";

		if (seconds == 1)
			return "1 second";

		ArrayList<String> duration = new ArrayList<>();

		if (seconds <= 3600) {
			duration.add(toHours(seconds));
			duration.add(toMins(seconds));
			duration.add(toSecs(seconds));
		} else if (seconds <= 86400) {
			duration.add(toDays(seconds));
			duration.add(toHours(seconds));
			duration.add(toMins(seconds));
			duration.add(toSecs(seconds));
		} else {
			duration.add(toYears(seconds));
			duration.add(toDays(seconds));
			duration.add(toHours(seconds));
			duration.add(toMins(seconds));
			duration.add(toSecs(seconds));
		}

		duration.removeIf(s -> s.isBlank());

		switch (duration.size()) {
			case 2:
				return duration.get(0) + " and " + duration.get(1);
			case 3:
				return duration.get(0) + ", " + duration.get(1) + " and " + duration.get(2);
			case 4:
				return duration.get(0) + ", " + duration.get(1) + ", " + duration.get(2) + " and " + duration.get(3);
			case 5:
				return duration.get(0) + ", " + duration.get(1) + ", " + duration.get(2) + ", " + duration.get(3)
						+ " and " + duration.get(4);
			default:
				return duration.get(0);
		}
	}

	public static void main(String[] args) {
		int[] sec = { 1, 62, 120, 3600, 3662, 132030240 };
		for (int s : sec) {
			System.out.println(formatDuration(s));
		}
	}
}
