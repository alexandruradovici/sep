#include "hello.h"
#include <timer.h>
 
int main(void) {
    while (true) {
        int nr = get_counter();
        printf ("Counter %d\n", nr);
        delay_ms(1000);
    }
    return 0;
}
