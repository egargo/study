# ITLympics 2023

USTP ITLymics 2023

# Problem 1 - Reverse Words

Complete the solution so that it reverses all of the words within the string passed in.

Words are separated by exactly one space and there are no leading or trailing spaces.

Example:

```js
reverse_words("The greatest victory is that which requires no battle") => "battle no requires which that is victory greatest The"
```

# Problem 2 - IPV4 to int32

Take the following IPv4 address: 128.32.10.1 This address has 4 octets where each octet is a single byte (or 8 bits).

-   1st octet `128` has the binary representation: `10000000`
-   2nd octet `32` has the binary representation: `00100000`
-   3rd octet `10` has the binary representation: `00001010`
-   4th octet `1` has the binary representation: `00000001`
    Therefore, `128.32.10.1` == `10000000.00100000.00001010.00000001`

The above IP address has 32 bits, we can represent it as the 32 bit number `2149583361`.

Write a function `ip_to_int32(ip)` that takes an IPv4 address and returns a 32 bit number.

Example:

```js
ip_to_int32("128.32.10.1") => 2149583361
```

# Problem 3 - Find the missing letter

Write a method that takes an array of consecutive (increasing) letters as input and that returns the missing letter in the array.

You will always get an valid array. And it will be always exactly one letter be missing. The length of the array will always be at least 2.
The array will always contain letters in only one case.

Example:

```js
find_missing_letter(['a','b','c','d','f']) => 'e'
find_missing_letter(['O','Q','R','S']) => 'P'
```

# Problem 4 - Two Sum

Write a function that takes an array of numbers (integers for the tests) and a target number. It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple / list (depending on your language) like so: (index1, index2).

For the purposes of this kata, some tests may have multiple answers; any valid solutions will be accepted.

The input will always be valid (numbers will be an array of length 2 or greater, and all of the items will be numbers; target will always be the sum of two different items from that array).

Based on: http://oj.leetcode.com/problems/two-sum/

```js
twoSum([1, 2, 3], 4) => [0, 2] // or [2, 0]
```
