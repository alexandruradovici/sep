#include "sevenseg.h"
#include <tock.h>

bool is_sevenseg_available(void)
{
    return command(SEVENSEG_DRIVER_NUMBER, 0, 0, 0) == TOCK_SUCCESS;
}
int turn_on_display(void)
{
    return command(SEVENSEG_DRIVER_NUMBER, 1, 0, 0) == TOCK_SUCCESS;
}
int turn_off_display(void)
{
    return command(SEVENSEG_DRIVER_NUMBER, 2, 0, 0) == TOCK_SUCCESS;
}
int set_number(int number)
{
    return command(SEVENSEG_DRIVER_NUMBER, 3, number, 0) == TOCK_SUCCESS;
}