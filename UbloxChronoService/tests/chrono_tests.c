//
// Created by rleroux on 9/19/24.
//

#include <assert.h>
#include <stdint.h>
#include <unistd.h>
#include "tests.h"
#include "chrono_tests.h"

#include "../chrono/chrono.h"
#include "../utils/timespec.h"

static struct chrono chrono = {
        .best_lap_time = 0,
        .previous_lap_time = 0,
        .current_lap_time = 0,
        .previous_sector_delta_time = 0,
        .best_lap_n = 0,
        .current_lap_n = 0
};

static struct gate_segment segments[] = {
        {.a = {.lon = 1, .lat = 2}, .b = {.lon = 3, .lat = 4} },
        {.a = {.lon = 5, .lat = 6}, .b = {.lon = 7, .lat = 8} },
        {.a = {.lon = -1, .lat = -2}, .b = {.lon = -3, .lat = -4} },
};
static struct sector_gates gates = {.current_index = 0, .count = 3, .segments = segments };

static struct chrono_context chrono_ctx = {.chrono = &chrono, .gates = &gates};

static void test_timespec_utils() {
    struct timespec t0, t1, diff, ts_from_tenths;
    clock_gettime(CLOCK_MONOTONIC_RAW, &t0);
    sleep(1);
    clock_gettime(CLOCK_MONOTONIC_RAW, &t1);

    diff = diff_timespec(&t1, &t0);

    assert(diff.tv_sec == 1);

    int32_t tenths = timespec_to_tenths(&diff);

    assert(tenths < 20);

    ts_from_tenths = tenths_to_timespec(tenths); // expected loss of precision

    assert(tenths == timespec_to_tenths(&ts_from_tenths));
}

static void test_pos_callback() {
    ts_coord pos1 = {
            .coord = { .lon = 2, .lat = 1 },
            .ts = tenths_to_timespec(123)
    };
    handle_position(pos1, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 123);

    // pass sector 1 gate
    ts_coord pos2 = {
            .coord = { .lon = 2, .lat = 5 },
            .ts = tenths_to_timespec(456)
    };
    handle_position(pos2, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 456);
    assert(chrono_ctx.chrono->current_lap_n == 0);
    assert(chrono_ctx.chrono->best_lap_time == 0);
    assert(chrono_ctx.chrono->best_lap_n == 0);
    assert(chrono_ctx.chrono->previous_lap_time == 0);
    assert(chrono_ctx.chrono->previous_sector_delta_time == 456);

    assert(chrono_ctx.gates->current_index == 1);

    assert(chrono_ctx.gates->segments[0].previous_time == 456);
    assert(chrono_ctx.gates->segments[0].best_time == 456);
    assert(chrono_ctx.gates->segments[0].best_time_lap_n == 0);

    // drive in sector 1
    ts_coord pos3 = {
            .coord = { .lon = 4, .lat = 7 },
            .ts = tenths_to_timespec(1000)
    };
    handle_position(pos3, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 1000);
    assert(chrono_ctx.chrono->current_lap_n == 0);

    assert(chrono_ctx.gates->current_index == 1);

    // pass sector 2 gate
    ts_coord pos4 = {
            .coord = { .lon = 8, .lat = 7 },
            .ts = tenths_to_timespec(1500)
    };
    handle_position(pos4, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 1500);
    assert(chrono_ctx.chrono->current_lap_n == 0);

    assert(chrono_ctx.gates->current_index == 2);

    assert(chrono_ctx.gates->segments[1].previous_time == 1500 - 456);
    assert(chrono_ctx.gates->segments[1].best_time == 1500 - 456);
    assert(chrono_ctx.gates->segments[1].best_time_lap_n == 0);

    // drive in sector 2
    ts_coord pos5 = {
            .coord = { .lon = 2, .lat = -3 },
            .ts = tenths_to_timespec(2200)
    };
    handle_position(pos5, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 2200);
    assert(chrono_ctx.chrono->current_lap_n == 0);

    assert(chrono_ctx.gates->current_index == 2);

    // pass sector 3 gate, start-finish line
    ts_coord pos6 = {
            .coord = { .lon = -3, .lat = -3 },
            .ts = tenths_to_timespec(3000)
    };
    handle_position(pos6, &chrono_ctx);

    assert(chrono_ctx.chrono->current_lap_time == 0);
    assert(chrono_ctx.chrono->current_lap_n == 1); // lap count increased
    assert(chrono_ctx.chrono->best_lap_time == 3000);
    assert(chrono_ctx.chrono->best_lap_n == 0);
    assert(chrono_ctx.chrono->previous_lap_time == 3000);

    assert(chrono_ctx.gates->current_index == 0); // back to sector 1

    assert(chrono_ctx.gates->segments[2].previous_time == 3000 - 1500);
    assert(chrono_ctx.gates->segments[2].best_time == 3000 - 1500);
    assert(chrono_ctx.gates->segments[2].best_time_lap_n == 0);

    // drive sector 1 lap 1
    ts_coord pos7 = {
            .coord = { .lon = -3, .lat = 3 },
            .ts = tenths_to_timespec(3100)
    };
    handle_position(pos7, &chrono_ctx);

    // pass sector 2 gate lap 1
    ts_coord pos8 = {
            .coord = { .lon = 4, .lat = 3 },
            .ts = tenths_to_timespec(3200)
    };
    handle_position(pos8, &chrono_ctx);

    // we finished the sector in 200 this time
    assert(chrono_ctx.chrono->previous_sector_delta_time == 200 - 456);
    assert(chrono_ctx.gates->segments[0].best_time == 200);
    assert(chrono_ctx.gates->segments[0].best_time_lap_n == 1);
}

void run_chrono_tests() {
    //TEST(test_timespec_utils)

    TEST(test_pos_callback)
}