// https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/javascript

function count(string) {
    let count = {};

    for (const i in string) {
        if (!(string[i] in count)) {
            count[string[i]] = 1;
        } else {
            count[string[i]] += 1;
        }
    }

    return count;
}

console.log(count("Bee"))
