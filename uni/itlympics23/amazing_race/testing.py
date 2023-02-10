#!/usr/bin/env python3

import unittest
from solution import VigenereCipher

latin = 'abcdefghijklmnopqrstuvwxyz'
katakana = 'アイウエオァィゥェォカキクケコサシスセソタチツッテトナニヌネノハヒフヘホマミムメモヤャユュヨョラリルレロワヲンー'

class Test(unittest.TestCase):
	def alpha_latin_encode(self):
		encode1 = {
			'itlympics': 'qmwwyeqek',
			'USTP': 'USTP',
			'debugging': 'lxmssvqpy',
			'ITLMYPICS': 'ITLMYPICS',
			'python': 'xrefac',
			'it\'s a shift cipher!': 'qm\'q p uzqye oxxjwz!',
			'linux': 'tbysj',
		}
		encode2 = {
			'amazingrace': 'ayayqamiaei',
			'AMAZINGRACE': 'AMAZINGRACE',
			'vigenere cipher': 'vugdvrxv empteq',
		}

		vc1 = VigenereCipher('itlympics', latin)
		print("Successful encoding test lowercase Latin alphabet with key of 'itlympics'")
		[self.assertEqual(vc1.encode(texts[0]), texts[1]) for texts in encode1.items()]

		vc2 = VigenereCipher('amazingrace', latin)
		print("Successful encoding test lowercase Latin alphabet with key of 'amazingrace'")
		[self.assertEqual(vc2.encode(texts[0]), texts[1]) for texts in encode2.items()]


	def alpha_latin_decode(self):
		decode1 = {
			'qmwwyeqek': 'itlympics',
			'USTP': 'USTP',
			'lxmssvqpy': 'debugging',
			'ITLMYPICS': 'ITLMYPICS',
			'xrefac': 'python',
			'qm\'q p uzqye oxxjwz!': 'it\'s a shift cipher!',
			'tbysj': 'linux',
		}
		decode2 = {
			'ayayqamiaei': 'amazingrace',
			'AMAZINGRACE': 'AMAZINGRACE',
			'vugdvrxv empteq': 'vigenere cipher',
		}

		vc1 = VigenereCipher('itlympics', latin)
		print("Successful decoding test lowercase Latin alphabet with key of 'itlympics'")
		[self.assertEqual(vc1.decode(texts[0]), texts[1]) for texts in decode1.items()]

		vc2 = VigenereCipher('amazingrace', latin)
		print("Successful decoding test lowercase Latin alphabet with key of 'amazingrace'")
		[self.assertEqual(vc2.decode(texts[0]), texts[1]) for texts in decode2.items()]

			

	def alpha_katakana_encode(self):
		encode = {
			'カタカナ': 'タモタワ',
			'ドモアリガトゴザイマス': 'ドオカセガヨゴザキアニ',
		}

		vc = VigenereCipher('カタカナ', katakana)
		print("Successful encoding test Katakana characters with key of 'カタカナ'")
		[self.assertEqual(vc.encode(texts[0]), texts[1]) for texts in encode.items()]


	def alpha_katakana_decode(self):
		decode = {
			'タモタワ': 'カタカナ',
			'ドオカセガヨゴザキアニ':'ドモアリガトゴザイマス',
		}

		vc = VigenereCipher('カタカナ', katakana)
		print("Successful decoding test Katakana characters with key of 'カタカナ'")
		[self.assertEqual(vc.decode(texts[0]), texts[1]) for texts in decode.items()]


		
	def test(self):
		self.alpha_latin_encode()
		self.alpha_latin_decode()
		self.alpha_katakana_encode()
		self.alpha_katakana_decode()





if __name__ == '__main__':
	unittest.main()