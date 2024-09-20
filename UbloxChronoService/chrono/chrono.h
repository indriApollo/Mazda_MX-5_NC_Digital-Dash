//
// Created by rleroux on 9/7/24.
//

#ifndef CHRONO_H
#define CHRONO_H

#include "../utils/coord.h"

struct gate_segment {
    coord a;
    coord b;
    uint32_t previous_time;
    uint32_t best_time;
    uint16_t best_time_lap_n;
};

struct sector_gates {
    int current_index;
    int count;
    struct gate_segment *segments;
};

struct chrono {
    // Timescale is tenths of a second
    uint32_t best_lap_time;
    uint32_t previous_lap_time;
    uint32_t current_lap_time;
    int32_t previous_sector_delta_time;
    uint16_t best_lap_n;
    uint16_t current_lap_n;
};

struct chrono_context {
    struct chrono *chrono;
    struct sector_gates *gates;
};

void handle_position(ts_coord pos, void *arg);

#endif //CHRONO_H
