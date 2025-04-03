#!/usr/bin/env python3

def parse(permission):
	octal = {
		"r": 4,
		"w": 2,
		"x": 1,
		"-": 0,
	}

	return sum([octal[i] for i in permission])

def chmod_calculator(perm):
	user = "0"
	group = "0"
	other = "0"

	if "user" in perm:
		user = parse(perm["user"])
	if "group" in perm:
		group = parse(perm["group"])
	if "other" in perm:
		other = parse(perm["other"])

	return f"{user}{group}{other}"

if __name__ == "__main__":
	perm = { "user": "rwx", "group": "r-x", "other": "r-x" }
	chmod = chmod_calculator(perm)
	print(chmod)
