#include <timer.h>
#include <stdio.h>
#include "sevenseg.h"

int main (void)
{
  seven_seg_on ();
  for (int position = 0; ;position++) {
    for (int number = 0; number < 10; number++) {
      seven_seg_write (number, position % 4);
      printf ("displaying %d at position %d\n", seven_seg_get_number(), seven_seg_get_position());
      delay_ms (2000);
    }
  }
}
