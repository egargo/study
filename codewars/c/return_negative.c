// https://www.codewars.com/kata/55685cd7ad70877c23000102/train/c

#include <stdio.h>

int makeNegative(int num) {
    if (num < 0) {
        return num;
    }

    return num * -1;
}

int main(int argc, char *argv[]) {
    printf("%d\n", makeNegative(2));
    printf("%d\n", makeNegative(12));
    printf("%d\n", makeNegative(22));
    printf("%d\n", makeNegative(0));
    printf("%d\n", makeNegative(-2));
    printf("%d\n", makeNegative(-12));
    printf("%d\n", makeNegative(-22));

    return 0;
}
