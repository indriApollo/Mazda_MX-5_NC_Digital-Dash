//
// Created by rleroux on 8/28/24.
//

#ifndef UBLOXGNSSSERVICE_COORD_H
#define UBLOXGNSSSERVICE_COORD_H

#include <stdint.h>
#include <time.h>

typedef struct {
    int32_t lon;
    int32_t lat;
} coord;

typedef struct {
    coord coord;
    struct timespec ts;
} ts_coord;

#endif //UBLOXGNSSSERVICE_COORD_H
