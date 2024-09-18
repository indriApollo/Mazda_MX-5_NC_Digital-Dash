//
// Created by rleroux on 9/7/24.
//

#include "chrono.h"

#include <stdio.h>
#include <time.h>

#include "../utils/intersect.h"
#include "../utils/timespec.h"

struct gate_segment {
    coord a;
    coord b;
};

struct gates {
    int current_index;
    int count;
    struct gate_segment segments[];
};

struct gates gates = {.current_index = 0, .count = 3, .segments = {
    {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
    {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
    {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
}};

coord previous_pos;
struct timespec previous_time;

void handle_position(const coord pos) {
    struct timespec now, sector_diff;
    clock_gettime(CLOCK_MONOTONIC_RAW, &now);

    printf("gps pos %d %d\n", pos.lon, pos.lat);

    const struct gate_segment current_gate_segment = gates.segments[gates.current_index];

    if (do_intersect(previous_pos, pos, current_gate_segment.a, current_gate_segment.b)) {
        sector_diff = diff_timespec(&now, &previous_time);
        previous_time = now;

        printf("sector %d diff %ld %ld", gates.current_index, sector_diff.tv_sec, sector_diff.tv_nsec / (long)1e6);
    }

    previous_pos = pos;
}

