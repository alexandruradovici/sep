#include <gpio.h>
#include <timer.h>
#include <math.h>
#include <stdlib.h>
#include <timer.h>
 
#define LATCH_DIO 4
#define CLK_DIO 7
#define DATA_DIO 8
 
void shift_out(GPIO_Pin_t data, GPIO_Pin_t clock, unsigned char value);
void setup(void);
void loop(int number);
void write_number(unsigned char segment, unsigned char value);
 
void shift_out(GPIO_Pin_t data, GPIO_Pin_t clock, unsigned char value) {
    gpio_enable_output(data);
    gpio_enable_output(clock);
 
    for (int i = 7; i >= 0; i--) {
        if (((value >> i) & 0x01) == 1) {
            gpio_set(data);
        } else {
            gpio_clear(data);
        }
        gpio_set(clock);
        gpio_clear(clock);
    }
}
 
const unsigned char SEGMENT_MAP[] = {0xC0, 0xF9, 0xA4, 0xB0, 0x99, 0x92, 0x82, 0xF8, 0X80, 0X90};
 
const unsigned char SEGMENT_SELECT[] = {0xF1, 0xF2, 0xF4, 0xF8};
 
void setup(void) {
    gpio_enable_output(LATCH_DIO);
    gpio_enable_output(CLK_DIO);
    gpio_enable_output(DATA_DIO);
}
 
void write_number(unsigned char segment, unsigned char value) {
    gpio_set(LATCH_DIO);
    shift_out(DATA_DIO, CLK_DIO, value);
    shift_out(DATA_DIO, CLK_DIO, SEGMENT_SELECT[segment]);
    gpio_clear(LATCH_DIO);
}
 
 
int main(void) {
    gpio_enable_output(LATCH_DIO);
    gpio_enable_output(CLK_DIO);
    gpio_enable_output(DATA_DIO);

    for (int number = 0; ; number++)
    {
        write_number(3, SEGMENT_MAP[number % 10]);
        delay_ms (3000);
    }
    return 0;
}