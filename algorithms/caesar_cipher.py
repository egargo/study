# Caesar Cipher


import operator

alphabet = {
	"A": 65, "B": 66, "C": 67, "D": 68, "E": 69, "F": 70, "G": 71, "H": 72,
	"I": 73, "J": 74, "K": 75, "L": 76, "M": 77, "N": 78, "O": 79, "P": 80,
	"Q": 81, "R": 82, "S": 83, "T": 84, "U": 85, "V": 86, "W": 87, "X": 88,
	"Y": 89, "Z": 90, "a": 97, "b": 98, "c": 99, "d": 100, "e": 101,
	"f": 102, "g": 103, "h": 104, "i": 105, "j": 106, "k": 107, "l": 108,
	"m": 109, "n": 110, "o": 111, "p": 112, "q": 113, "r": 114, "s": 115,
	"t": 116, "u": 117, "v": 118, "w": 119, "x": 120, "y": 121, "z": 122
}

operators = {
	'+': [operator.add, operator.gt, operator.sub],
	'-': [operator.sub, operator.lt, operator.add],
}


def decrypt(message, op):
	encrypted = ""

	for i in message:
		if i in alphabet.keys():
			check = operators[op][0](alphabet.get(i), 1)
			if i.isupper():
				if operators[op][0](alphabet.get(i))[0](65):
					encrypted += chr(check + 26)
				else:
					encrypted += chr(check)
			else:
				if (alphabet.get(i) - 1) < 97:
					encrypted += chr(check + 26)
				else:
					encrypted += chr(check)
		else:
			encrypted += i

	return encrypted


def encrypt(message):
	encrypted = ""

	for i in message:
		if i in alphabet.keys():
			check = alphabet.get(i) + 1
			if i.isupper():
				if (alphabet.get(i) + 1) > 90:
					encrypted += chr(check - 26)
				else:
					encrypted += chr(check)
			else:
				if (alphabet.get(i) + 1) > 122:
					encrypted += chr(check - 26)
				else:
					encrypted += chr(check)
		else:
			encrypted += i

	return encrypted


if __name__ == "__main__":
	message = input()
	print(encrypt(message))
	message = input()
	print(decrypt(message, '-'))