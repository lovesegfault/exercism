#include "armstrong_numbers.h"
#include <math.h>
#include <stdlib.h>

bool is_armstrong_number(int candidate) {
    int num_len = floor(log10((double) abs(candidate))) + 1;

    int sum = 0;
    for (int i = candidate; i > 0; i /= 10) {
        int current_digit = i % 10;
        sum += pow(current_digit, num_len);
    }

    return (sum == candidate);
}
