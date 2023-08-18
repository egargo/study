# https://www.codewars.com/kata/55968ab32cf633c3f8000008/train/python


def initials(name):
	name = name.title().split(' ')

	return f"{''.join([f'{i[0]}.' for i in name[:-1]])}{name[-1]}"


if __name__ == '__main__':
	print(initials('code wars'))
	print(initials('Barack hussein obama'))
	print(initials('barack hussein Obama'))
	print(initials('Walter Hartwell White'))
