// https://www.codewars.com/kata/541a354c39c5efa5fa001372/train/typescript

export function ipToNum(ip: string): number {
    return parseInt(ip.split(".").map((x) => {
        return Number(x).toString(2).padStart(8, "0");
    }).join(""), 2);
}

export function numToIp(x: number): string {
    return [...Array(4)].map((_, i) => {
        return Math.floor((x / Math.pow(256, i)) % 256);
    }).reverse().join(".");
}

let num: number = ipToNum("192.168.1.1");
let ip: string = numToIp(num);

console.log(num);
console.log(ip);
