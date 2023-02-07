import unittest
from solution import reversed_words

class Test(unittest.TestCase):
	def test(self):
		data = {
			'hello world!': 'world! hello',
			'yoda doesn\'t speak like this': 'this like speak doesn\'t yoda',
			'foobar': 'foobar',
			'kata editor':'editor kata',
			'row row row your boat':'boat your row row row',
		}
		for i in data.items():
			result = reversed_words(i[0])
			self.assertEqual(i[1], result)

if __name__ == '__main__':
	unittest.main()