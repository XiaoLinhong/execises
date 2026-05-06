#include "doctor_data.h"

namespace heaven {
    std::string get_older_bob(heaven::Vessel a, heaven::Vessel b) {
        if (a.generation < b.generation) {
            return a.name;
        } else {
            return b.name;
        }
    } 

    bool in_the_same_system(heaven::Vessel a, heaven::Vessel b) {
        return a.current_system == b.current_system;
    }
}