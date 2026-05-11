#include "queen_attack.h"
#include <cstdlib>

namespace queen_attack {

  bool chess_board::can_attack() const {
    if (_white.first == _black.first 
      || _white.second == _black.second 
      || std::abs(_white.second - _black.second) == std::abs(_white.first - _black.first)
    ) {
        return true;
      }
    return false;
  }

}  // namespace queen_attack
