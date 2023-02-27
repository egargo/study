package study.oop.polymorphism;

class Insect {
	public void fly() {
		System.out.println("Fly!");
	}

	public String fly(String s) {
		return s + " flew away!";
	}
}
