//
// Created by rleroux on 8/14/24.
//

#ifndef TESTS_H
#define TESTS_H

#include <stdio.h>

#define RED   "\x1B[31m"
#define GRN   "\x1B[32m"
#define YEL   "\x1B[33m"
#define BLU   "\x1B[34m"
#define MAG   "\x1B[35m"
#define CYN   "\x1B[36m"
#define WHT   "\x1B[37m"
#define RESET "\x1B[0m"

#define TEST(fn) printf(MAG "Running %s\n" RESET, #fn); fn(); printf(GRN "%s complete\n" RESET, #fn);

#endif //TESTS_H
