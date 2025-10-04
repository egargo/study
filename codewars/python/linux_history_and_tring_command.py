#!/usr/bin/env python3

# https://www.codewars.com/kata/5818236ae7f457017b00022b/train/python


import re


def bang_start_string(s, history):
	for h in history.split("\n")[::-1]:
		_, c = h.strip(" ").split("  ")
		if (re.match(pattern=s, string=c) is not None) and s in c:
			return c
	return f"!{s}: event not found"


if __name__ == "__main__":
	history="   1  cd /pub\n  2  more beer\n  3  lost\n  4  ls\n  5  touch me\n  6  chmod 000 me \n  7  more me\n  8  history"

	assert bang_start_string("more", history) == "more me"
	assert bang_start_string("mkdir", history) == "!mkdir: event not found"
