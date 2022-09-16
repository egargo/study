def solution(lst):

	even = [i for i in lst if i % 2 == 0]
	odd = [i for i in lst if i % 2 != 0]


	sum_even = sum([i for i in even if i % 2 == 0])
	sum_odd = sum([i for i in odd if i % 2 != 0])

	return [sum_even, sum_odd]


if __name__ == '__main__':

	print(solution([1,2,3,4,5,6,7,8,9,10,11,12,13,14]))