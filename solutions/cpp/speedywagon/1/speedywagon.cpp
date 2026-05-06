#include "speedywagon.h"

namespace speedywagon {

// Enter your code below:

// Please don't change the interface of the uv_light_heuristic function
int uv_light_heuristic(std::vector<int>* data_array) {
    double avg{};
    for (auto element : *data_array) {
        avg += element;
    }
    avg /= data_array->size();
    int uv_index{};
    for (auto element : *data_array) {
        if (element > avg) ++uv_index;
    }
    return uv_index;
}

bool uv_alarm(pillar_men_sensor* sensor){
    if ( !connection_check(sensor) ){
        return false;
    }

    int n = uv_light_heuristic( &(sensor->data) );

    if (sensor->activity > n){
        return false;
    }
    return true;
}
 
bool connection_check(const pillar_men_sensor* sensor){
    if (sensor == nullptr) {
        return false;
    }
    return true;
}

int activity_counter(const pillar_men_sensor* sensor, int capacity){
    int n {0};
    for (int i=0; i<capacity; i++) {
        // n += (*(sensor+i)).activity;
        // n += (sensor + i)->activity;
        n += sensor[i].activity;
    }
    return n;
}

bool alarm_control(const pillar_men_sensor* sensor){
    if ( !connection_check(sensor) ){
        return false;
    }
    if (sensor->activity > 0){
        return true;
    }
    return false;
}

}  // namespace speedywagon 
