#include "lasagna_master.h"
#include <cstddef>

namespace lasagna_master {


int preparationTime(const std::vector<std::string>& recipe, int minutes) {
    return  recipe.size() * minutes;
}

amount quantities(const std::vector<std::string>& recipe) {
    int noodles {0};
    double sauce {0.0};

    for (size_t i=0; i < recipe.size(); i++) {
        if (recipe[i] == "noodles") {
            noodles += 50;
        }
        if (recipe[i] == "sauce") {
            sauce += 0.2;
        }
    }
    return amount {noodles, sauce};
}

void addSecretIngredient(std::vector<std::string>& recipe, const std::vector<std::string>& new_recipe){
    // recipe.pop_back();
    // const std::string& last = new_recipe.back();
    // recipe.emplace_back(last); // recipe.push(last.clone());
    recipe.back() = new_recipe.back();
}

std::vector<double> scaleRecipe(const std::vector<double>& quantities, int n){
    std::vector<double> new_quantities;

    for (size_t i=0; i < quantities.size(); i++) {
        new_quantities.emplace_back(quantities[i]*0.5*n);
    }
    return new_quantities;
}

void addSecretIngredient(std::vector<std::string>& recipe, const std::string& secretIngredient) {
    recipe.back() = secretIngredient;
    // recipe.emplace_back(secretIngredient);
}

}  // namespace lasagna_master