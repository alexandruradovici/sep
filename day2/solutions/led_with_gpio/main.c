#include <stdio.h>
#include <timer.h>
#include <gpio.h>

int main(void) {
    int num_count = gpio_count();
    printf("There are %d\n", num_count);
    for (int i = 10; i < 18; ++i) {
        gpio_enable_output(i);
        gpio_set(i);
    }
    while (1) {
        for (int i = 10; i < 18; ++i) {
            gpio_clear(i);
            printf("Starting led %d\n", i);
            delay_ms(500);

            printf("Closing led %d\n", i); 
            gpio_set(i);
            delay_ms(500);
        }
    }
    return 0;
}