// https://www.codewars.com/kata/514b92a657cdc65150000006/train/java

public class MultiplesOf3Or5 {
	public static void main(String[] args) {
		MultiplesOf3Or5 multiplesOf3Or5 = new MultiplesOf3Or5();
		System.out.println(multiplesOf3Or5.solution(10));
	}

	public int solution(int number) {
		int sum = 0;

		for (int i = 0; i < number; ++i) {
			if (i % 5 == 0 || i % 3 == 0) {
				sum += i;
			}
		}

		return sum;
	}
}
