package study.oop.inheritance;

public class Insect {
	public String name;

	public Insect(String name) {
		this.name = name;
	}

	public String fly() {
		return "The " + this.name + " flew away!";
	}
}
