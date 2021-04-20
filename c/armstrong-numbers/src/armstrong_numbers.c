#include "armstrong_numbers.h"
#include <math.h>

bool is_armstrong_number(int candidate) {
    int acc = candidate;
    int num_len = 0;
    while (acc > 0) {
        num_len++;
        acc /= 10;
    }

    int digits[num_len];
    acc = candidate;
    num_len = 0;
    while (acc > 0) {
        int current_digit = acc % 10;
        digits[num_len] = current_digit;
        num_len++;
        acc /= 10;
    }

    int sum = 0;
    for (int i = 0; i < num_len; ++i) {
        sum += pow(digits[i], num_len);
    }

    return (sum == candidate);
}
