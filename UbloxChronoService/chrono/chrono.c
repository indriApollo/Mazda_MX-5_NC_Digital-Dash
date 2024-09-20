//
// Created by rleroux on 9/7/24.
//

#include "chrono.h"

#include <stdio.h>
#include <time.h>

#include "../utils/intersect.h"
#include "../utils/timespec.h"

coord previous_pos_coord;
struct timespec sector_start_time;
struct timespec lap_start_time;

void handle_position(const ts_coord pos, void *arg) {
    struct chrono_context *ctx = arg;

    //printf("gps pos %d %d\n", pos.coord.lon, pos.coord.lat);

    struct timespec lap_time = diff_timespec(&pos.ts, &lap_start_time);
    ctx->chrono->current_lap_time = timespec_to_tenths(&lap_time);

    struct gate_segment *current_gate_segment = &ctx->gates->segments[ctx->gates->current_index];

    bool passed_sector_gate = do_intersect(previous_pos_coord, pos.coord, current_gate_segment->a, current_gate_segment->b);

    if (passed_sector_gate) {
        struct timespec sector_ts = diff_timespec(&pos.ts, &sector_start_time);
        int32_t sector_time = timespec_to_tenths(&sector_ts);
        sector_start_time = pos.ts;

        ctx->chrono->previous_sector_delta_time = (int32_t)(sector_time - current_gate_segment->previous_time);

        bool new_best_sector = sector_time < current_gate_segment->best_time
                || current_gate_segment->best_time == 0;

        if (new_best_sector) {
            current_gate_segment->best_time = sector_time;
            current_gate_segment->best_time_lap_n = ctx->chrono->current_lap_n;
        }

        current_gate_segment->previous_time = sector_time;

        printf("sector %d diff %d\n", ctx->gates->current_index, ctx->chrono->previous_sector_delta_time);

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

            lap_start_time = pos.ts;

            ctx->gates->current_index = 0;
        }
        else {
            ctx->gates->current_index++;
        }
    }

    previous_pos_coord = pos.coord;
}

