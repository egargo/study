#!/usr/bin/env python3


# https://www.codewars.com/kata/52d1bd3694d26f8d6e0000d3/train/python


class VigenereCipher(object):
	def __init__(self, key, alphabet):
		self.key = key
		self.alphabet = alphabet
		self.ALPHA = {self.alphabet[i]: i for i in range(len(self.alphabet))}
		self.LENGTH = len(self.alphabet)


	def get_key_by_value(self, value):
		return [i[0] for i in self.ALPHA.items() if value == i[1]][0]


	def check_length(self, text):
		KEY = self.key

		if len(self.key) > len(text):
			KEY = KEY[0:len(text)]
		else:
			for i in range(len(text) - len(KEY)):
				KEY += KEY[i]

		return KEY


	def encode(self, text):
		if text.isupper():
			return text

		KEY = self.check_length(text)

		encoded = ''

		for i in range(len(text)):
			if self.ALPHA.get(text[i]) == None:
				encoded += text[i]
			if self.ALPHA.get(KEY[i]) == None:
				encoded += KEY[i]
			if self.ALPHA.get(text[i]) != None and self.ALPHA.get(KEY[i]) != None:
				check = (((self.ALPHA.get(text[i]) + self.ALPHA.get(KEY[i]))) % self.LENGTH)
				encoded += (self.get_key_by_value(check))

		return encoded


	def decode(self, text):
		if text.isupper():
			return text

		KEY = self.check_length(text)

		decoded = ''

		for i in range(len(text)):

			if self.ALPHA.get(text[i]) == None:
				decoded += text[i]
			if self.ALPHA.get(KEY[i]) == None:
				decoded += KEY[i]
			if self.ALPHA.get(text[i]) != None and self.ALPHA.get(KEY[i]) != None:
				check = (((self.ALPHA.get(text[i]) - self.ALPHA.get(KEY[i]))) % self.LENGTH)
				decoded += (self.get_key_by_value(check))

		return decoded


if __name__ == '__main__':
	# '''
	key = 'password'
	abc = 'abcdefghijklmnopqrstuvwxyz'
	text = "it's a shift cipher!"
	# '''

	# '''
	key = 'カタカナ'
	abc = 'アイウエオァィゥェォカキクケコサシスセソタチツッテトナニヌネノハヒフヘホマミムメモヤャユュヨョラリルレロワヲンー'
	text = 'カタカナ'
	# '''

	vc = VigenereCipher(key, abc)
	enc = vc.encode(text)

	print('Encoded : ', enc)
	print('Decoded : ', vc.decode(enc))