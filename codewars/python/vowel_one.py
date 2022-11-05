# https://www.codewars.com/kata/580751a40b5a777a200000a1/train/python


def vowel_one(s):
	return ''.join(['1' if i in 'aeiouAEIOU' else '0' for i in s])


if __name__ == '__main__':
	s = input()
	# abceios
	# 1001110
	# 1001110


	print(vowel_one(s))