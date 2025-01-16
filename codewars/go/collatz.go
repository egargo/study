// https://www.codewars.com/kata/5286b2e162056fd0cb000c20/train/go

package main

import (
	"strconv"
	"strings"
)

func Collatz(n int) string {
	sequence := strconv.Itoa(n)
	sequence += "->"

	for n != 1 {
		if n%2 == 0 {
			n = n / 2
		} else {
			n = n*3 + 1
		}

		sequence += strconv.Itoa(n)
		sequence += "->"
	}

	return strings.TrimSuffix(sequence, "->")
}

func main() {
	c := Collatz(12)
	println(c)
}
