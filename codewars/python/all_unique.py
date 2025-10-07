# https://www.codewars.com/kata/553e8b195b853c6db4000048/train/python


def has_unique_chars(string):
	char_counter = {}

	for i in string:
		if i in char_counter.keys():
			char_counter[i] += 1
		else:
			char_counter[i] = 0

	return True if 1 not in char_counter.values() else False
