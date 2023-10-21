// https://www.codewars.com/kata/515e271a311df0350d00000f/train/javascript

function squareSum(numbers) {
    let sum = 0;

    numbers.forEach(element => {
        sum += Math.pow(element, 2);
    });

    return sum;
}

console.log(squareSum([1, 2, 2]));
