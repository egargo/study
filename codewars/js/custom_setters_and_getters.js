// www.codewars.com/kata/5241060ff2471a5d7600025a/train/javascript

class Archiver {
    constructor() {
        this.logs = [];
        this.temp = 0;
    }

    set temperature(temperature) {
        this.temp = temperature;
        this.logs.push({ date: new Date(), val: this.temp });
    }

    get temperature() {
        return this.temp;
    }

    getArchive() {
        return this.logs;
    }
}
