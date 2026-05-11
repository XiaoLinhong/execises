#include "sieve.h"
#include <set>

namespace sieve {


bool is_prime(int n ) {
  for (int i = 2; i*i<=n; i++) {
    if (n % i == 0) {
        return false;
    }
  }
  return true;
}

std::vector<int> primes(int n){
    std::vector<int> unmarked;
    std::set<int> marked;

    for(int i = 2; i<=n; i++) {
        if (marked.count(i)) {
            continue;
        }
        if (is_prime(i)) {
            unmarked.emplace_back(i);
            for (int j=2; j<=n; j++) {
                if (j*i <=n) {
                    marked.insert(j*i);
                } else {
                    break;
                }
            }
        }
    }

    return unmarked;
}

}  // namespace sieve
 