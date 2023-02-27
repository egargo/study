package study.oop.polymorphism;


class Main {
	public static void main(String[] args) {
		Insect insect = new Insect();
		insect.fly();

		Insect bee = new Bee();
		System.out.println(bee.fly("Bee"));
	}
}
