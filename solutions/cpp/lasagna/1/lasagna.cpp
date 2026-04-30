// ovenTime returns the amount in minutes that the lasagna should stay in the
// oven.
constexpr int EXPECTED_MINUTES_IN_OVEN = 40;
constexpr int PREPARATION_TIME_PER_LAYER = 2;
int ovenTime() {
    // TODO: Return the correct time.
    return EXPECTED_MINUTES_IN_OVEN;
}

/* remainingOvenTime returns the remaining
   minutes based on the actual minutes already in the oven.
*/
int remainingOvenTime(int actualMinutesInOven) {
    // TODO: Calculate and return the remaining in the oven based on the time
    // the lasagna has already been there.
    return EXPECTED_MINUTES_IN_OVEN - actualMinutesInOven;
}

/* preparationTime returns an estimate of the preparation time based on the
   number of layers and the necessary time per layer.
*/
int preparationTime(int numberOfLayers) {
    // TODO: Calculate and return the preparation time with the
    // `numberOfLayers`.
    return numberOfLayers * PREPARATION_TIME_PER_LAYER;
}

// elapsedTime calculates the total time spent to create and bake the lasagna so
// far.
int elapsedTime(int numberOfLayers, int actualMinutesInOven) {
    // TODO: Calculate and return the total time so far.
    return preparationTime(numberOfLayers) + actualMinutesInOven;
}
