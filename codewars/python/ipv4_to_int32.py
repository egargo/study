#!/usr/bin/env python3

def ip_to_int32(ip):
	return sum([(pow(256, i[0]) * i[1]) for i in enumerate(list(reversed([int(i) for i in ip.split('.')])))])



if __name__ == '__main__':
	ip = '128.32.10.1'
	print(ip_to_int32(ip))