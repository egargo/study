// https://www.codewars.com/kata/52f2f5ecc807c0ab1a00001a/train/javascript


let latin1 = 'abcdefghijklmnopqrstuvwxyz';
let latin2 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
let hebrew = 'אבגדהוזחטיכלמנסעפצקרשת';
let greek = 'ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ';

let abc = hebrew;

class AtbashCipher {
	constructor(abc) {
		const alphabet = new Map([]);
		this.LENGTH = abc.length;

		let vc = this.LENGTH - 1;
		for(let i = 0; i < this.LENGTH / 2; ++i) {
			alphabet.set(abc[i], abc[vc]);
			vc -= 1;
		}


		this.get_key_by_value = function(char) {
			for(const [k, v] of alphabet.entries()) {
				if(v === char) {
					return k;
				}
			}
		}

		this.check_map = function(str) {
			let crypt = '';

			for(let i = 0; i < str.length; ++i) {
				if(alphabet.get(str[i]) === undefined)
					crypt += this.get_key_by_value(str[i]);
				else
					crypt += alphabet.get(str[i]);
			}

			return crypt;
		}

		this.encode = function (str) {
			if(str.match(/[A-Z]/))
				return str;

			return this.check_map(str);
		};


		this.decode = function (str) {
			if(str.match(/[A-Z]/))
				return str;

			return this.check_map(str);
		};
	}
}


let c = new AtbashCipher(abc);
let str = 'אבג';

console.log(c.encode(str));