// https://www.codewars.com/kata/541a354c39c5efa5fa001372/train/javascript

function ipToNum(ip) {
    return parseInt(ip.split(".").map((x) => {
        return Number(x).toString(2).padStart(8, "0");
    }).join(""), 2);
}

function numToIp(num) {
    return [...Array(4)].map((_, i) => {
        return Math.floor((num / Math.pow(256, i)) % 256);
    }).reverse().join(".");
}

let num = ipToNum("192.168.1.1");
let ip = numToIp(num);

console.log(num);
console.log(ip);
