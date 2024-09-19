cmake_minimum_required(VERSION 3.0)
project(Mx5MetricsService C)

set(CMAKE_C_STANDARD 17)

add_executable(Mx5MetricsService main.c
        metrics.h
        metrics.c
        serial_port.c
        serial_port.h
        stnobd.c
        stnobd.h)
