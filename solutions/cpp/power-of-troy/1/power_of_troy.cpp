#include "power_of_troy.h"

#include <memory>

namespace troy {
void give_new_artifact(human& person, const std::string name){
    person.possession = std::make_unique<artifact>(artifact{name});
}

void exchange_artifacts(std::unique_ptr<artifact>& a1, std::unique_ptr<artifact>& a2){
    // if (a1 == nullptr) {
    //     if (a2 == nullptr) {
    //         return;
    //     } else {
    //         a1 = std::make_unique<artifact>(artifact{a2->name});
    //         a2 = nullptr;
    //         return;
    //     }
    // }

    // if (a2 == nullptr) {
    //     a2 = std::make_unique<artifact>(artifact{a1->name});
    //     a1 = nullptr;
    //     return;
    // }

    // auto name = a1->name;
    // a1->name = a2->name;
    // a2->name = name;
    std::swap(a1, a2);
}

void manifest_power(human& person, const std::string name){
    person.own_power = std::make_shared<power>(power{name});
}

void use_power(const human& p1, human& p2){
    p2.influenced_by = p1.own_power;
}

int power_intensity(const human& p1) {
    if (p1.own_power == nullptr) {
        return 0;
    }
    return p1.own_power.use_count();
}

}  // namespace troy
