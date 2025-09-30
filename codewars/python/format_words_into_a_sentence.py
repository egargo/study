# https://www.codewars.com/kata/51689e27fe9a00b126000004/train/python


def format_words(words):
	if (words is None or len(words) == 0) or (len(words) == 1 and words[0] == ""):
		return ""

	words = [i for i in words if i != ""]

	if len(words) == 1:
		return "".join(words)

	if len(words) == 2:
		return f"{words[0]} and {words[1]}"

	return f"{', '.join(words[0:-1])} and {words[-1]}"


if __name__ == "__main__":
	print(format_words(['ninja', 'samurai', 'ronin']))
