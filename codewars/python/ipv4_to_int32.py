#!/usr/bin/env python3

# https://www.codewars.com/kata/52ea928a1ef5cfec800003ee/train/python

def ip_to_int32(ip):
    return sum([(pow(256, i[0]) * i[1]) for i in enumerate(reversed([int(i) for i in ip.split('.')]))])

print(ip_to_int32("128.114.17.104"))
print(ip_to_int32("0.0.0.0"))
print(ip_to_int32("128.32.10.1"))
