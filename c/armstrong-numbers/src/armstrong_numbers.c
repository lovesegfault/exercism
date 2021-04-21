#include "armstrong_numbers.h"
#include <math.h>

bool is_armstrong_number(int candidate) {
    if (candidate < 0) {
        return false;
    } else if (candidate == 0) {
        return true;
    }

    int num_len = log10(candidate) + 1;

    int sum = 0;
    for (int i = candidate; i > 0; i /= 10) {
        int current_digit = i % 10;
        sum += pow(current_digit, num_len);
    }

    return (sum == candidate);
}
