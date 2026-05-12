#pragma once

#include <map>
#include <string>
#include <optional>
#include <vector>

namespace alphametics {

std::optional<std::map<char, int>> solve(const std::string& puzzle);

std::optional<std::vector<std::string>> tokenize(const std::string& puzzle);

}
// namespace alphametics
