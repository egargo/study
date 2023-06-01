#!/usr/bin/env python3

def solution():
    inputs = []

    while 42 not in inputs:
        inputs.append(int(input()))

    return inputs[:-1]


if __name__ == '__main__':
    for i in solution():
        print(i)
