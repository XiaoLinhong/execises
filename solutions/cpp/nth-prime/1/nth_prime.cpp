#include "nth_prime.h"
#include <cmath>
#include <stdexcept>

namespace nth_prime {

bool is_prime(int n) {
    int factor = static_cast<int>(std::sqrt(n));
    for (int i=2; i<=factor; i++) {
        if (n%i == 0) {
            return false;
        }
    }
    return true;
}

int nth(int n){

    if (n < 1) {
        throw std::domain_error("smaller than 1");
    }

    int i {0};
    int prime {1};
    while(i < n) {
        prime++;
        if (is_prime(prime)) {
            i++;
        }
    }
    return prime;
}

}  // namespace nth_prime
