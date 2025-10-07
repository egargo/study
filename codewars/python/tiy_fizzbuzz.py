# https://www.codewars.com/kata/5889177bf148eddd150002cc/train/python


def tiy_fizz_buzz(string):
	vowels = ["a", "i", "e", "o", "u"]

	if string == " ":
		return " "

	fizzbuzz = ""

	for i in string:
		if i.isupper() and i.lower() not in vowels:
			fizzbuzz += "Iron"
		elif i.isupper() and i.lower() in vowels:
			fizzbuzz += "Iron Yard"
		elif i.islower() and i.lower() in vowels:
			fizzbuzz += "Yard"
		else:
			fizzbuzz += i

	return fizzbuzz
