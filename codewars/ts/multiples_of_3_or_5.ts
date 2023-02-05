// https://www.codewars.com/kata/514b92a657cdc65150000006/train/typescript

export class Challenge {
	static solution(number: number) {
		let sum: number = 0;

		for (let i = 0; i < number; ++i)
			sum += i % 3 === 0 || i % 5 === 0 ? i : 0;

		return sum;
	}
}

console.log(Challenge.solution(10));
