//
// Created by rleroux on 9/13/24.
//

#include "timespec.h"

// https://stackoverflow.com/a/68804612
struct timespec diff_timespec(const struct timespec *t1, const struct timespec *t0) {
    struct timespec diff = {
        .tv_sec = t1->tv_sec - t0->tv_sec,
        .tv_nsec = t1->tv_nsec - t0->tv_nsec
    };

    if (diff.tv_nsec < 0) {
        diff.tv_nsec += 1000000000; // nsec/sec
        diff.tv_sec--;
    }

    return diff;
}

int32_t timespec_to_tenths(struct timespec *timespec) {
    int32_t tenth = (int32_t)timespec->tv_sec * 10;
    tenth += (int32_t)(timespec->tv_nsec / (long)1e8);
    return tenth;
}

struct timespec tenths_to_timespec(int32_t tenths) {
    struct timespec timespec = {
            .tv_sec = tenths / 10,
            .tv_nsec = (tenths % 10) * (long)1e8
    };
    return timespec;
}
