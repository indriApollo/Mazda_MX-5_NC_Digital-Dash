//
// Created by rleroux on 9/7/24.
//

#include "chrono.h"

#include <stdio.h>
#include <time.h>

#include "../utils/intersect.h"
#include "../utils/timespec.h"

coord previous_pos;
struct timespec previous_time;

void handle_position(const coord pos, void *arg) {
    struct chrono_context *ctx = arg;
    struct timespec now, sector_diff;
    clock_gettime(CLOCK_MONOTONIC_RAW, &now);

    printf("gps pos %d %d\n", pos.lon, pos.lat);

    struct gate_segment *current_gate_segment = &ctx->gates->segments[ctx->gates->current_index];

    if (do_intersect(previous_pos, pos, current_gate_segment->a, current_gate_segment->b)) {
        sector_diff = diff_timespec(&now, &previous_time);
        previous_time = now;

        printf("sector %d diff %ld %ld", ctx->gates->current_index, sector_diff.tv_sec, sector_diff.tv_nsec / (long)1e6);
    }

    previous_pos = pos;
}

