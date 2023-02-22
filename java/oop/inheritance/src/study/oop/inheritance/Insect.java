package study.oop.inheritance;

public class Insect {
	public String name;

	public Insect(String name) {
		this.name = name;
	}

	public String fly() {
		// System.out.println(String.format("{0} flew away!", this.name));
		return "The " + this.name + " flew away!";
	}
}
