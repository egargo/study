// https://www.codewars.com/kata/534d2f5b5371ecf8d2000a08/train/typescript


export const multiplicationTable = (size: number): number[][] => {
	let table: number[][] = [];
	let temp: number[] = [];

	for(let i = 1; i <= size; ++i) {
		for(let j = 1; j <= size; ++j) {
			temp.push(i*j);
		}

		table.push(temp);
		temp = [];
	}

	return table;
}


console.log(multiplicationTable(12));