#pragma once

#include <string>

namespace star_map {
    enum System {
        Sol,
        BetaHydri,
        EpsilonEridani,
        AlphaCentauri,
        DeltaEridani,
        Omicron2Eridani,
    };
}

namespace heaven {
    class Vessel {
        public:
            std::string name;
            int generation;
            star_map::System current_system;
            int busters {0};
            Vessel(std::string user_name, int user_generation , star_map::System user_current_system  = star_map::System::Sol) {
                name = user_name;
                generation  = user_generation ;
                current_system  = user_current_system ;
            }
            Vessel replicate(std::string other_name) {
                return Vessel {other_name, generation +1};
            }
            void make_buster(){
                busters ++;
            }
            bool shoot_buster() {
                if (busters > 0) {
                    busters--;
                    return true;
                }
                return false;
            }
    };

    std::string get_older_bob(heaven::Vessel a, heaven::Vessel b);
    bool in_the_same_system(heaven::Vessel a, heaven::Vessel b);

}