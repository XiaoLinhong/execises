#pragma once
#include <utility>
#include <stdexcept>

namespace queen_attack {
    class chess_board {
        private:
        std::pair<int,int> _white;
        std::pair<int,int> _black;

        public:
        chess_board(std::pair<int,int> white, std::pair<int,int> black) {
            if (white.first < 0 || white.second < 0 || white.first > 7 || white.second > 7 
             || black.first < 0 || black.second < 0 || black.first > 7 || black.second > 7) {
                throw std::domain_error("out of board");
            }
            if (white.first == black.first && white.second == black.second) {
                throw std::domain_error("some position");
            }
            _white = white;
            _black = black;
        }

        // 构造函数函数初始化列表
        // chess_board(std::pair<int,int> white, std::pair<int,int> black): _white(white), _black(black) {}

        std::pair<int,int> white() const {
            return _white;
        }

        std::pair<int,int> black() const {
            return _black;
        }

        bool can_attack() const;
    };
 
}  // namespace queen_attack
