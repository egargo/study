package study.oop.abstraction;

public class Bee extends Insect {
	@Override
	public void fly() {
		System.out.println("The Bee flew away!");
	}

	@Override
	public void sting() {
		System.out.println("The Bee stung you!");
	}
}
