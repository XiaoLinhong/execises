#include "alphametics.h"
#include <cmath>
#include <cstddef>
#include <iostream>
#include <set>
#include <string>
#include <vector>
// #include <sstream>

namespace alphametics {

bool is_delim(char c) {
    return c == ' ' || c == '+' || c == '=';
}

std::optional<std::vector<std::string>> tokenize(const std::string& puzzle) {

    if (puzzle.find("+") == std::string::npos || puzzle.find("=") == std::string::npos) {
        return std::nullopt;
    }
    std::vector<std::string> result;

    std::string item {""};

    for (auto c: puzzle) {
        if (is_delim(c)) {
            if (!item.empty()) {
                result.push_back(item);
                item.clear();
            }

        } else {
            item += c;
        }
    }
    if (!item.empty()) {
        result.push_back(item);
    }
    return result;
}

int value(std::map<char, int>& dict, std::string& word) {
    int n {0};
    int d (1);
    for(size_t i=word.size(); i > 0; i--) {
        n += dict[word[i-1]] * d;
        d *= 10;
    }
    return n;
}

bool recursion(std::map<char, int>& dict, std::map<char, bool>& head, std::set<int>& choiced, std::vector<char>& left, std::vector<std::string>& words) {
    if (left.empty()) {
        int n {0};
        for (size_t j=0; j<words.size(); j++){
            if (j < words.size()-1) {
                n += value(dict, words[j]);
            } else {
                n -= value(dict, words[j]);
            }
        }
        if (n == 0) {
            return true;
        }
        return false;
    }
    auto c = left[0];
    auto choiced_new = choiced;
    std::vector<char> next_left(left.begin() + 1, left.end());
    for (int i=0; i<10; i++) {
        if (head[c] && i==0) continue;
        if (choiced.find(i) != choiced.end()) continue; // 已经有了

        choiced_new.insert(i);
        dict[c] = i;
        if (recursion(dict, head, choiced_new, next_left, words)) {
            return true;
        }
        choiced_new.erase(i);
    }
    return false;
}

bool update(std::map<char, int>& dict, std::map<char, bool>& head, std::vector<std::string>& words) {
    std::set<int> choiced;
    std::vector<char> left;
    for (auto &[c, _] : dict) {
        left.push_back(c);
    }
    return recursion(dict, head, choiced, left, words);
}

std::optional<std::map<char, int>> solve(const std::string& puzzle) {
    std::map<char, int> dict;
    std::map<char, bool> head;
    auto words = tokenize(puzzle);
    if (!words.has_value()) {
        return std::nullopt;
    }
    auto ww = *words;

    for (const auto& word: ww) {
        head[word[0]] = true;
        for (auto c: word) {
            dict[c] = -1;
        }
    }

    for (auto &[c, _] : dict) {
        head[c] = false;
    }

    for (const auto& word: ww) {
        head[word[0]] = true;
    }

    if (update(dict, head, ww)) {
        return dict;
    }
    return std::nullopt;
}

}  // namespace alphametics
 