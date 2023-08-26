// https://www.codewars.com/kata/514b92a657cdc65150000006/train/go

package main

import "fmt"

func Multiple3And5(number int) int {
    var sum = 0;

    for i := 1; i < number; i++ {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}

func main() {
    var number = 0;

    fmt.Scanf("%d", &number);

    fmt.Println(Multiple3And5(number));
}
