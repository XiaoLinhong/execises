#include <string>

namespace log_line {
    std::string message(std::string line) {
        int idx = line.find(": ");
        return line.substr(idx + 2);
    }

    std::string log_level(std::string line) {
        int beg = line.find("[") + 1;
        int end = line.find("]");
        return line.substr(beg, end - beg);
    }

    std::string reformat(std::string line) {
        std::string lvl = log_level(line);
        std::string msg = message(line);
        return msg + " (" + lvl + ")";
    }
}  // namespace log_line
