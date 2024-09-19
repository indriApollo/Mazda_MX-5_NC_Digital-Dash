//
// Created by rleroux on 9/7/24.
//

#include "chrono.h"

#include <stdio.h>
#include <time.h>

#include "../utils/intersect.h"
#include "../utils/timespec.h"

coord previous_pos;
struct timespec sector_start_time;
struct timespec lap_start_time;

static int32_t timespec_to_tenth(struct timespec *timespec) {
    int32_t tenth = (int32_t)timespec->tv_sec * 10;
    tenth += (int32_t)(timespec->tv_nsec / (long)1e8);
    return tenth;
}

void handle_position(const coord pos, void *arg) {
    struct chrono_context *ctx = arg;
    struct timespec now, sector_diff;
    // TODO timestamping a pos like this is not super accurate,
    // but as we only care about tenths of a second it might no even matter
    clock_gettime(CLOCK_MONOTONIC_RAW, &now);

    printf("gps pos %d %d\n", pos.lon, pos.lat);

    struct timespec lap_time = diff_timespec(&now, &lap_start_time);
    ctx->chrono->current_lap_time = timespec_to_tenth(&lap_time);

    struct gate_segment *current_gate_segment = &ctx->gates->segments[ctx->gates->current_index];

    if (do_intersect(previous_pos, pos, current_gate_segment->a, current_gate_segment->b)) {
        sector_diff = diff_timespec(&now, &sector_start_time);
        sector_start_time = now;

        ctx->chrono->previous_sector_delta_time = timespec_to_tenth(&sector_diff);

        printf("sector %d diff %d", ctx->gates->current_index, ctx->chrono->previous_sector_delta_time);

        bool passed_start_finish_line = ctx->gates->current_index >= ctx->gates->count - 1;

        if (passed_start_finish_line) {
            bool new_best_lap = ctx->chrono->current_lap_time < ctx->chrono->best_lap_time
                    || ctx->chrono->best_lap_time == 0;

            if (new_best_lap) {
                ctx->chrono->best_lap_time = ctx->chrono->current_lap_time;
                ctx->chrono->best_lap_n = ctx->chrono->current_lap_n;
            }

            ctx->chrono->previous_lap_time = ctx->chrono->current_lap_time;
            ctx->chrono->current_lap_time = 0;
            ctx->chrono->current_lap_n++;

            lap_start_time = now;
        }
    }

    previous_pos = pos;
}

