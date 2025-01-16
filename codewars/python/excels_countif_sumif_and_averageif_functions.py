#!/usr/bin/env python3

# https://www.codewars.com/kata/56055244356dc5c45c00001e


import operator
import re


def parse_criteria(criteria):
	operators = {
		"<": operator.lt,
		"<=": operator.le,
		">": operator.gt,
		">=": operator.ge,
		"<>": operator.ne,
	}

	o = re.compile(">=|<=|<>|<|>").search(criteria)
	n = re.sub("[>|>=|<|<=|<>]+", "", criteria).strip()

	if "." in n:
		n = float(n)
	else:
		n = int(n)

	if o is None:
		return None

	if n is None:
		return None

	return (operators[o.group(0).strip()], n)


def count_if(values, criteria):
	if isinstance(criteria, str):
		parsed = parse_criteria(criteria)
		if parsed is not None:
			return len([i for i in values if parsed[0](i, parsed[1])])
	return len([i for i in values if i == criteria])

def sum_if(values, criteria):
	if isinstance(criteria, str):
		parsed = parse_criteria(criteria)
		if parsed is not None:
			return sum([i for i in values if parsed[0](i, parsed[1])])
	return sum([i for i in values if i == criteria])


def average_if(values, criteria):
	filtered = []
	if isinstance(criteria, str):
		parsed = parse_criteria(criteria)
		if parsed is not None:
			filtered = [i for i in values if parsed[0](i, parsed[1])]
	else:
		filtered = [i for i in values if i == criteria]

	return sum(filtered) / len(filtered)


if __name__ == "__main__":
	o = count_if([1,3,5,3],3)
	print(o)
	o = count_if([1.5,3,5,3,0,-1,-5],'<1.5')
	print(o)
	o = count_if([1.5,3,5,3,0,-1,-5],'<=1.5')
	print(o)
	o = count_if([1.5,3,5,3,0,-1,-5],'>1.5')
	print(o)
	o = count_if([1.5,3,5,3,0,-1,-5],'>=1.5')
	print(o)
	o = count_if([1.5,3,5,3,0,-1,-5],'<>1.5')
	print(o)
