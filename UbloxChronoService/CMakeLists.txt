cmake_minimum_required(VERSION 3.0)
project(UbloxChronoService C)

set(CMAKE_C_STANDARD 17)

add_executable(UbloxChronoService main.c
        utils/fletcher8.c
        utils/fletcher8.h
        serial/serial_port.c
        serial/serial_port.h
        ublox/ublox.c
        ublox/ublox.h
        ublox/ublox.c
        tests/ublox_tests.c
        tests/ublox_tests.h
        tests/tests.h
        utils/intersect.c
        utils/intersect.h
        utils/coord.h
        utils/buffer.c
        utils/buffer.h
        chrono/chrono.c
        chrono/chrono.h
        utils/timespec.c
        utils/timespec.h
        tests/chrono_tests.c
        tests/chrono_tests.h)
