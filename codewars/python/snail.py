#!/usr/bin/env python3

# https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1/train/python

def snail(snail_map):
	sort = []

	for _ in range(0, len(snail_map)):
		rotation = 1

		if rotation == 1 and len(snail_map) != 0:
			for i in snail_map[0]:
				sort.append(i)
			del snail_map[0]
			rotation += 1

		if rotation == 2:
			for i in range(0, len(snail_map)):
				sort.append(snail_map[i][-1])
				del snail_map[i][-1]
			rotation += 1

		if rotation == 3 and len(snail_map) != 0:
			snail_map[-1].reverse()
			for i in (snail_map[-1]):
				sort.append(i)
			del snail_map[-1]
			rotation += 1

		if rotation == 4 and len(snail_map) != 0:
			for i in range(len(snail_map)-1, -1, -1):
				sort.append(snail_map[i][0])
				del snail_map[i][0]
			rotation += 1

	return sort


if __name__ == "__main__":
	array = [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15], [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]]
	sort = snail(array)
	expected = [1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12, 13]
	print(f"Ok: {sort == expected}")
	print(f"Result: {sort}")
