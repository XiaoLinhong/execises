#include <array>
#include <cstddef>
#include <string>
#include <vector>

// Round down all provided student scores.
std::vector<int> round_down_scores(std::vector<double> student_scores) {
    std::vector<int> scores;
    for (int i {0}; i < student_scores.size(); i++) {
        scores.emplace_back(static_cast<int>(student_scores[i]));
    }

    return scores;
}

// Count the number of failing students out of the group provided.
int count_failed_students(std::vector<int> student_scores) {
    int n {0};
    for (int score: student_scores) {
        if (score <= 40) {
            n++;
        }
    }
    return n;
}

// Create a list of grade thresholds based on the provided highest grade.
std::array<int, 4> letter_grades(int highest_score) {
    // TODO: Implement letter_grades
    int gap = (highest_score - 40)/4;
    std::array<int, 4> thresholds {41, 41 + gap, 41 + gap*2, 41 + gap*3 };
    return thresholds;
}

// Organize the student's rank, name, and grade information in ascending order.
std::vector<std::string> student_ranking(
    std::vector<int> student_scores, std::vector<std::string> student_names) {
    std::vector<std::string> order;
    for (size_t i = 0; i < student_scores.size(); ++i) {
        int score = student_scores[i];
        std::string& name = student_names[i]; // 从rust所有权的角度来解释发生了什么，string被拷贝了嘛？还是这只是一个指针？
        order.emplace_back(std::to_string(i+1) + ". " + name + ": " + std::to_string(score) );
    }
    return order;
}

// Create a string that contains the name of the first student to make a perfect
// score on the exam.
std::string perfect_score(std::vector<int> student_scores,
                          std::vector<std::string> student_names) {
    std::string name {""};
    for (size_t i {0}; i < student_scores.size(); i++ ) {
        if (student_scores[i] == 100) {
            name += student_names[i];
            break;
        }
    }
    return name;
}
