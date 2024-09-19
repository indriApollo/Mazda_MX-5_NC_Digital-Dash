//
// Created by rleroux on 9/13/24.
//

#ifndef UBLOXGNSSSERVICE_TIMESPEC_H
#define UBLOXGNSSSERVICE_TIMESPEC_H

#include <time.h>

struct timespec diff_timespec(const struct timespec *t1, const struct timespec *t0);

#endif //UBLOXGNSSSERVICE_TIMESPEC_H
