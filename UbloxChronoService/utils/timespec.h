//
// Created by rleroux on 9/13/24.
//

#ifndef UBLOXGNSSSERVICE_TIMESPEC_H
#define UBLOXGNSSSERVICE_TIMESPEC_H

#include <time.h>
#include <stdint.h>

struct timespec diff_timespec(const struct timespec *t1, const struct timespec *t0);

int32_t timespec_to_tenths(struct timespec *timespec);

struct timespec tenths_to_timespec(int32_t tenths);

#endif //UBLOXGNSSSERVICE_TIMESPEC_H
