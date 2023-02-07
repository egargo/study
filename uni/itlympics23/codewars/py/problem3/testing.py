import unittest
from solution import find_missing_letter

class Test(unittest.TestCase):
	def test(self):
		data = {
			('a', 'b', 'c', 'd', 'f') :'e',
			('O', 'Q', 'R', 'S') :'P',
			('b', 'd') :'c',
			('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'S', 'T', 'U', 'V', 'W', 'X', 'Y') :'R',
			('S', 'T', 'U', 'W', 'X') :'V',
			('b', 'c', 'e', 'f') :'d',
			('q', 's', 't', 'u', 'v', 'w') :'r',
			('G', 'H', 'I', 'J', 'L') :'K',
			('W', 'Y', 'Z') :'X',
			('e', 'f', 'g', 'i', 'j', 'k', 'l', 'm', 'n') :'h',
			('P', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y') :'Q',
			('L', 'M', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V') :'N',
			('S', 'T', 'V', 'W', 'X', 'Y', 'Z') :'U',
			('g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 's', 't', 'u', 'v') :'r',
			('N', 'O', 'Q', 'R') :'P',
			('e', 'f', 'g', 'h', 'i', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u') :'j',
			('A', 'B', 'D', 'E') :'C',
			('t', 'u', 'v', 'x', 'y', 'z') :'w',
			('p', 'q', 's') :'r',
			('N', 'O', 'P', 'Q', 'R', 'S', 'T', 'V') :'U',
			('r', 's', 't', 'u', 'v', 'x', 'y') :'w',
			('d', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 's', 't', 'u', 'v', 'w', 'x', 'y') :'r',
			('n', 'o', 'p', 'q', 'r', 's', 't', 'v', 'w') :'u',
			('o', 'p', 'r', 's', 't', 'u') :'q',
			('e', 'g', 'h', 'i', 'j') :'f',
			('k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't') :'o',
			('O', 'P', 'R') :'Q',
			('G', 'H', 'I', 'J', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S') :'K',
			('G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T') :'I',
			('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z') :'M',
			('I', 'J', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T') :'K',
			('P', 'Q', 'R', 'T', 'U') :'S',
			('v', 'x', 'y') :'w'
		}

		for i, j in data.items():
			result = find_missing_letter(list(i))
			self.assertEqual(j, result)

if __name__ == '__main__':
	unittest.main()