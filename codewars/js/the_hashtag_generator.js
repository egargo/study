// https://www.codewars.com/kata/52449b062fb80683ec000024/train/javascript

function generateHashtag (str) {
    str = str.replace(/\s+/g, " ").trim();

    if((str === "") || (str.trim().replace(" ", "").length === 0 ||
        (str.length == 0) || (str.length > 140))) return false;

    let hashtag = "#";

    str.split(" ").forEach(element => {
        hashtag += `${element[0].toUpperCase()}${element.slice(1,)}`;
    });

    return hashtag.length <= 140 ? hashtag : false;
}

console.log(generateHashtag(""));
console.log(generateHashtag("DoWeHaveAHashtag"));
console.log(generateHashtag(" ".repeat(20)));
console.log(generateHashtag("code                                                                                                                                            wars"));
console.log(generateHashtag("a".repeat(139)));
console.log(generateHashtag("a".repeat(140)));
console.log(generateHashtag("Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong"));
console.log(generateHashtag(" Hello there thanks for trying my Kata "));
