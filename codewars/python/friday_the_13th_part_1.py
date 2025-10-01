#!/usr/bin/env python3

# https://www.codewars.com/kata/5925acf31a9825d616000e74/train/python


def kill_count(counselors, jason):
    return [counselor[0] for counselor in counselors if counselor[1] < jason]
