#include "hello.h"
#include <tock.h>

static void my_callback_function(int data1, int data2, int data3, void *user_data) {
	printf("callback called, counter is %d\r\n", data1);
}

int main(void)
{
	void* user_data = NULL;
	subscribe(HELLO_DRIVER_NUMBER, 0, my_callback_function, user_data);
	command(HELLO_DRIVER_NUMBER, 3, 0, 0);
}