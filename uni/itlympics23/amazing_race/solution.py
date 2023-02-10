import unittest
import operator

class VigenereCipher(object):
	def __init__(self, key, alphabet):
		self.key = key
		self.alphabet = alphabet
		self.alpha = {self.alphabet[i]: i for i in range(len(self.alphabet))}
		self.length = len(self.alphabet)
		self.op = {'-': operator.add, '+': operator.sub}

	def check_length(self, text):
		rem = (len(text) % len(self.key)) if len(text) < len(self.key) else (len(self.key) % len(text))
		return self.key * rem if rem != 0 else self.key

	def process(self, text, op):
		if text.isupper(): return text
		cipher = ''
		KEY = self.check_length(text)
		for i in range(len(text)):
			if self.alpha.get(text[i]) == None: cipher += text[i]
			if self.alpha.get(KEY[i]) == None: cipher += KEY[i]
			if self.alpha.get(text[i]) != None and self.alpha.get(KEY[i]) != None:
				check = (self.op.get(op)(self.alpha.get(text[i]), self.alpha.get(KEY[i])) % self.length)
				cipher += [i[0] for i in self.alpha.items() if check == i[1]][0]
		return cipher

	def encode(self, text):
		return text if text.isupper() else self.process(text, '+')

	def decode(self, text):
		return text if text.isupper() else self.process(text, '-')