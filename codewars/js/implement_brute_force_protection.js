// https://www.codewars.com/kata/586a514cc66d18807f0000a6/train/javascript


let rateLimit = {};

function bruteForceDetected(loginRequest) {
    const ip = loginRequest.sourceIP;
    const loggedIn = loginRequest.successful;

    if (!(ip in rateLimit)) {
        rateLimit[ip] = 1;
    }

    if ((ip in rateLimit) && !loggedIn) {
        rateLimit[ip] += 1;
    } else {
        rateLimit[ip] = 1;
    }

    return true ? rateLimit[ip] > 20 : false;
}
