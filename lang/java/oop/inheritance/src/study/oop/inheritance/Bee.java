package study.oop.inheritance;

public class Bee extends Insect {
	public Bee(String name) {
		super(name);
	}

	public String buzz() {
		return "The " + name + " says buzz!";
	}
}
