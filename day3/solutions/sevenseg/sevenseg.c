#include "sevenseg.h"
#include <tock.h>

#define SEVEN_SEG_NUMBER 0xa0000001

int seven_seg_on (void) {
    return command (SEVEN_SEG_NUMBER, 1, 0, 0);
}
int seven_seg_off (void) {
    return command (SEVEN_SEG_NUMBER, 2, 0, 0);
}
int seven_seg_write (int number, int position) {
    return command (SEVEN_SEG_NUMBER, 3, number, position);
}
int seven_seg_get_number (void) {
    return command (SEVEN_SEG_NUMBER, 4, 0, 0);
}
int seven_seg_get_position (void) {
    return command (SEVEN_SEG_NUMBER, 5, 0, 0);
}