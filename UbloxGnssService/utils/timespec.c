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
