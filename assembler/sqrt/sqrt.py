#!/usr/bin/env python3

def square(num):
    x1 = num / 2
    # x1(целая часть) + 0.5|0.0 + 0.5 ->  (x1(целая часть) + {0|1}x1(дробная)) >> 1
    x2 = (x1 + (num / 2)) // 2
    while(x1 - x2 >= 1):
        x1 = x2
        x2 = (x1 + (num / x1)) // 2
    return x2

if __name__ == "__main__":
    print(square(144))