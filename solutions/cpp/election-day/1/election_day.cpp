#include <cstddef>
#include <string>
#include <vector>

namespace election {

// The election result struct is already created for you:

struct ElectionResult {
    public:
    // Name of the candidate
    std::string name{};
    // Number of votes the candidate has
    int votes{};
};

int vote_count(ElectionResult& hamilton) {
    return hamilton.votes;
}

void increment_vote_count(ElectionResult& hamilton, int votes) {
    hamilton.votes += votes;
}

ElectionResult& determine_result(std::vector<ElectionResult>& candidates) {
    size_t best {0};
    for (size_t i=1; i<candidates.size(); i++) {
        if (candidates[best].votes < candidates[i].votes) {
            best=i;
        }
    }
    ElectionResult& president {candidates[best]};
    president.name = "President " + president.name;
    return president;
}

}  // namespace election
