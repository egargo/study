# https://www.codewars.com/kata/55acfc59c3c23d230f00006d/train/python


def get_ascii(ch: str) -> int:
	return ord(ch)


if __name__ == "__main__":
	for i in [ 'A', '\0', 'a', '0', '\n' ]:
		print(get_ascii(i))
