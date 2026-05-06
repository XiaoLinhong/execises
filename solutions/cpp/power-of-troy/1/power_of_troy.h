#pragma once

#include <string>
#include <memory>
namespace troy {

struct artifact {
    // constructors needed (until C++20)
    artifact(std::string name) : name(name) {}
    std::string name;
};

struct power {
    // constructors needed (until C++20)
    power(std::string effect) : effect(effect) {}
    std::string effect;
};

class human {
    public:
    std::unique_ptr<artifact> possession;
    std::shared_ptr<power> own_power;
    std::shared_ptr<power> influenced_by;

    human(){
        possession = nullptr;
        own_power = nullptr;
        influenced_by = nullptr;
    }

};

void give_new_artifact(human& person, const std::string name);
void exchange_artifacts(std::unique_ptr<artifact>& a1, std::unique_ptr<artifact>& a2);

void manifest_power(human& person, const std::string name);

void use_power(const human& p1, human& p2);

int power_intensity(const human& p1);

}  // namespace troy
 