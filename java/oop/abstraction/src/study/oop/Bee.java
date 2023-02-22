package study.oop;

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
