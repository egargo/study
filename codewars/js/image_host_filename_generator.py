# https://www.codewars.com/kata/586a933fc66d187b6e00031a/train/python

from datetime import datetime
import random

def generateName():
	timestamp = str(datetime.now().timestamp()).replace(".", "")[::-1][0:6]

	return "".join([chr(random.randint(65, 90)) if int(i) % 2 == 0 else chr(random.randint(97, 122)) for i in timestamp])

if __name__ == "__main__":
	print(generateName())
