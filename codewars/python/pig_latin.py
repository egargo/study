#!/usr/bin/env python3

def pig_it(text):
	pl = ''
	for i in text.split(' '):
		if i not in "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~":
			first = i[0]
			i = i.replace(i[0], '', 1)
			pl += f'{i}{first}ay'
		else:
			pl += i
		pl += ' '

	return pl.strip()

if __name__ == '__main__':
	print(pig_it('Pig latin is cool'))
	print(pig_it('Hello world !'))
