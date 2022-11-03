class VigenereCipher(object):
	def __init__(self, key, alphabet):
		self.key		= key
		self.alphabet	= alphabet
		self.LENGTH		= len(alphabet)
		self.BASE		= ord(alphabet[0])
		self.TOP		= ord(alphabet[len(alphabet) - 1])


	def encode(self, text):
		if text.isupper():
			return text

		KEY = self.key

		if len(self.key) > len(text):
			KEY = KEY[0:len(text)]
		else:
			for i in range(len(text) - len(KEY)):
				KEY += KEY[i]

		ascii_key = [(ord(i) - self.BASE) for i in KEY]
		ascii_txt = [(ord(i)) if (ord(i) - self.BASE) < 0 else (ord(i) - self.BASE) for i in text]

		encoded = ''

		for (i, j) in enumerate(ascii_key):
			check = ((ascii_txt[i] + j) + self.BASE)
			if (ascii_txt[i] >= 32 and ascii_txt[i] <= 64):
				encoded += chr(ascii_txt[i])
			else:
				if check > self.TOP:
					while check > self.TOP:
						check -= self.LENGTH
					encoded += chr((check))
				else:
					encoded += chr((check))

		return encoded


	def decode(self, text):
		if text.isupper():
			return text

		KEY = self.key

		if len(KEY) > len(text):
			for i in range(len(KEY) - len(text)):
				text += ' '
		else:
			for i in range(len(text) - len(KEY)):
				KEY += KEY[i]

		ascii_key = [(ord(i) - self.BASE) for i in KEY]
		ascii_txt = [(ord(i)) if (ord(i) - self.BASE) < 0 else (ord(i) - self.BASE) for i in text]

		decoded = ''

		for (i, j) in enumerate(ascii_key):
			check = ((ascii_txt[i] - j) + self.BASE)

			if check < self.BASE:
				check += self.LENGTH
			if ascii_txt[i] >= 32 and ascii_txt[i] <= 64:
				decoded += chr(ascii_txt[i])
			else:
				decoded += chr(check)

		return decoded.strip()


if __name__ == '__main__':
	key = 'password'
	abc = 'abcdefghijklmnopqrstuvwxyz'
	text = "it's a shift cipher!"

	vc = VigenereCipher(key, abc)
	enc = vc.encode(text)

	print('Encoded : ', enc)
	print('Decoded : ', vc.decode(enc))