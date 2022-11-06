// https://www.codewars.com/kata/5361372e700d2a9627000cf1/train/javascript


class KamaSutraCipher {
	constructor(key) {
		const KEY = new Map([]);
		for(let i = 0; i < key.length; ++i) {
			KEY.set(key[i][0], key[i][1]);
		}

		this.get_key_by_value = function(char) {
			for(const[k, v] of KEY.entries()) {
				if(v === char) return k;
			}

			return char;
		}

		this.check_map = function(str) {
			let crypt = '';

			for(let i = 0; i < str.length; ++i) {
				if(KEY.get(str[i]) !== undefined) crypt += KEY.get(str[i]);
				else crypt += this.get_key_by_value(str[i]);
			}

			return crypt;
		}

		this.encode = function (str) {
			if(str.match(/[A-Z]/)) return str;

			return this.check_map(str);
		};

		this.decode = function (str) {
			if(str.match(/[A-Z]/)) return str;

			return this.check_map(str);
		};
	}
}


let key = [
	['d', 'p'],
	['n', 'o'],
	['a', 'w'],
	['f', 'c'],
	['h', 's'],
	['l', 'v'],
	['m', 'j'],
	['x', 'b'],
	['e', 'z'],
	['r', 'i'],
	['k', 'y'],
	['u', 'q'],
	['t', 'g']
];

key = [["a","z"],["b","y"],["c","x"],["d","w"],["e","v"],["f","u"],["g","t"],["h","s"],["i","r"],["j","q"],["k","p"],["l","o"],["m","n"]];

key = [["a", "z"]]

let str = 'apple'

let vc = new KamaSutraCipher(key);


console.log(vc.encode(str))