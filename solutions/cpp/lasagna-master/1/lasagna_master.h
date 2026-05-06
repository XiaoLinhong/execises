#pragma once

#include <vector>
#include <string>

namespace lasagna_master {

struct amount {
    public:
    int noodles{};
    double sauce{};
};

int preparationTime(const std::vector<std::string>& recipe, int minutes = 2);

amount quantities(const std::vector<std::string>& recipe);

void addSecretIngredient(std::vector<std::string>& recipe, const std::vector<std::string>& new_recipe);

std::vector<double> scaleRecipe(const std::vector<double>& recipe, int n);

void addSecretIngredient(std::vector<std::string>& recipe, const std::string& secretIngredient);

}  // namespace lasagna_master
