#pragma once

#include <stdbool.h>

#define SEVENSEG_DRIVER_NUMBER 0xb0000000

bool is_sevenseg_available(void);
int turn_on_display(void);
int turn_off_display(void);
int set_number(int number);