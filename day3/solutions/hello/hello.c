 #include "hello.h"
#include <tock.h>
 
int hello_is_available(void) {
    return command(HELLO_DRIVER_NUMBER, 0, 0, 0) == TOCK_SUCCESS;
}
 
int signal_one(void) {
    return command(HELLO_DRIVER_NUMBER, 1, 0, 0);
}
 
int get_counter(void) {
    return command(HELLO_DRIVER_NUMBER, 2, 0, 0);
}