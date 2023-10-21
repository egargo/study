// https://www.codewars.com/kata/55a70521798b14d4750000a4/train/go

package main

import "fmt"

func Greet(name string) string {
    return "Hello, " + name + " how are you doing today?"
}

func main() {
    var name string;
    fmt.Scanf("%s", &name);
    fmt.Println(Greet(name));
}
