#pragma once

#include <stdbool.h>

#define HELLO_DRIVER_NUMBER 0xa0000000

bool hello_is_available(void);
int call_hello_1(void);
int increment_counter(void);