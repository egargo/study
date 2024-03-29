// https://www.codewars.com/kata/517abf86da9663f1d2000003/train/javascript

function toCamelCase(str) {
    let arr = [];
    let temp = "";

    for (let i = 0; i < str.length; ++i) {
        if (!["_", "-"].includes(str[i])) {
            temp += str[i];
        } else {
            arr.push(temp);
            temp = "";
        }
    }
    arr.push(temp);

    let camelCase = arr[0];

    for (let i = 1; i < arr.length; ++i) {
        for (let j = 0; j < arr[i].length; ++j) {
            if (j == 0) {
                camelCase += arr[i][j].toUpperCase();
            } else {
                camelCase += arr[i][j];
            }
        }
    }

    return camelCase;
}

console.log(toCamelCase("the-stealth-warrior"));
