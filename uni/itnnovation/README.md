# Problems
# Contents
* [First Year](#first-year)
  * [Easy](#fizzbuzz)
  * [Medium](#multiples-of-3-or-5)
  * [Hard](#power-digit-sum)
* [Second Year](#second-year)
  * [Easy](#find-the-missing-letter)
  * [Medium](#two-sum)
  * [Hard](#smallest-multiple)
* [Third Year](#third-year)
  * [Easy](#string-average)
  * [Medium](#collatz-conjecture)
  * [Hard](#sum-square-difference)
* [Fourth Year](#fourth-year)
  * [Easy](#uncollapse-digits)
  * [Medium](#highest-scoring-word)
  * [Hard](#1001st-prime)

# First Year
## Easy
### FizzBuzz
Return an array containing the numbers from 1 to N, where N is the parametered value.\
Replace certain values however if any of the following conditions are met:
-   If the value is a multiple of 3: use the value "Fizz" instead
-   If the value is a multiple of 5: use the value "Buzz" instead
-   If the value is a multiple of 3 & 5: use the value "FizzBuzz" instead
N will never be less than 1.

Method calling example:
```py
fizzbuzz(3) --> [1, 2, "Fizz"]
```
## Medium
### Multiples of 3 or 5
If we list all the natural numbers below `10` that are multiples of `3` or `5`, we get `3`, `5`, `6` and `9`. The sum of these multiples is 23.\
Find the sum of all the multiples of `3` or `5` below `1000`.
### Hard
#### Power digit sum
$2^{15} = 32768$ and the sum of its digits is $3 + 2 + 7 + 6 + 8 = 26$.\
What is the sum of the digits of the number $2^{1000}$?

# Second Year
### Easy
#### Find the missing letter
Write a method that takes an array of consecutive (increasing) letters as input and that returns the missing letter in the array.\
You will always get an valid array. And it will be always exactly one letter be missing. The length of the array will always be at least 2.\
The array will always contain letters in only one case.\
Example:
```
['a','b','c','d','f'] -> 'e'
['O','Q','R','S'] -> 'P'
```
## Medium
### Two Sum
Write a function that takes an array of numbers (integers for the tests) and a target number. It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple / list (depending on your language) like so: `(index1, index2)`.
```py
two_sum([1, 2, 3], 4)   # returns [0, 2] or [2, 0]
```
## Hard
### Smallest multiple
`2520` is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.\
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?


# Third Year
## Easy
### String Average
You are given a string of numbers between 0-9. Find the average of these numbers and return it as a floored whole number (ie: no decimal places) written out as a string. Eg:\
`zero nine five two` -> `four`\
If the string is empty or includes a number greater than 9, return `n/a`.
## Medium
### Collatz Conjecture
**Preface**
A collatz sequence, starting with a positive integern, is found by repeatedly applying the following function to n until n == 1 :
$$
\begin{equation}
f(n) = \begin{cases}
\frac{n}{2}, & \text{if } n \text{ is even} \\
3n + 1, & \text{if } n \text{ is odd}
\end{cases}
\end{equation}
$$
**The Problem**
Create a function `collatz` that returns a collatz sequence string starting with the positive integer argument passed into the function, in the following form:
`X0->X1->...->XN`
Where $X_{i}$ is each iteration of the sequence and $N$ is the length of the sequence.
**Sample Output**
```
Input: 4
Output: "4->2->1"

Input: 3
Output: "3->10->5->16->8->4->2->1"
```
## Hard
### Sum square difference
The sum of the squares of the first ten natural numbers is:
> $1^{2} + 2^{2} + ... + 10^{2} = 385$

The square of the sum of the first ten natural numbers is:
> $(1 + 2 + ... + 10)^{2} = 55^{2} = 3025$

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
> $3025 - 385 = 2640$

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.


# Fourth Year
## Easy
### Uncollapse Digits
**Task**\
You will be given a string of English digits "stuck" together, like this:\
`zeronineoneoneeighttwoseventhreesixfourtwofive`\
Your task is to split the string into separate digits:\
`zero nine one one eight two seven three six four two five`
**Examples**
```
"three"                                      -->  "three"
"eightsix"                                   -->  "eight six"
"fivefourseven"                              -->  "five four seven"
"ninethreesixthree"                          -->  "nine three six three"
"fivethreefivesixthreenineonesevenoneeight"  -->  "five three five six three nine one seven one eight"
```
## Medium
### Highest Scoring Word
Given a string of words, you need to find the highest scoring word.\
Each letter of a word scores points according to its position in the alphabet: `a = 1, b = 2, c = 3` etc.\
For example, the score of `abad` is `8` $(1 + 2 + 1 + 4)$.\
You need to return the highest scoring word as a string.
If two words score the same, return the word that appears earliest in the original string.\
All letters will be lowercase and all inputs will be valid.
## Hard
### 1001st prime
By listing the first six prime numbers: $2, 3, 5, 7, 11, and \space 13$, we can see that the 6th prime is 13.\
What is the $10,001st$ prime number?
