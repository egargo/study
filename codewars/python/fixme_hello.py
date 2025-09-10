#!/usr/bin/env python3

# https://www.codewars.com/kata/5b0a80ce84a30f4762000069/train/python


from typing import OrderedDict


class Dinglemouse:
	def __init__(self):
		self.message = OrderedDict()
		self.name = None
		self.sex = None
		self.age = None

	def setAge(self, age):
		self.message["age"] = age
		return self

	def setSex(self, sex):
		sexes = {"M": "male", "F": "female"}
		self.message["sex"] = sexes[sex]
		return self

	def setName(self, name):
		self.message["name"] = name
		return self

	def hello(self):
		message = ["Hello."]
		for k, v in self.message.items():
			if v is not None:
				if k in ["age", "sex"]:
					message.append(f"I am {v}.")
				else:
					message.append(f"My name is {v}.")
		return " ".join(message)



if __name__ == "__main__":
	dm1 = Dinglemouse().setName("Bob").setAge(27).setSex("M")
	dm2 = Dinglemouse().setAge(27).setSex("M").setName("Bob")
	dm3 = Dinglemouse().setName("Alice").setSex("F")
	dm4 = Dinglemouse().setName("Batman")
	dm5 = Dinglemouse().setName("Name 1").setAge(24).setName("Name 2")

	print(dm1.hello())
	print(dm2.hello())
	print(dm3.hello())
	print(dm4.hello())
	print(dm5.hello())
